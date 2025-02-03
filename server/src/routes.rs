use axum::{
    extract::{Path, State},
    response::{Json, Redirect},
    http::StatusCode,
    response::IntoResponse,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use chrono::{Utc, Duration, DateTime};
use tokio::time;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateUrl {
    target_url: String,
}

#[derive(Serialize, FromRow)]
pub struct UrlResponse {
    id: String,
    target_url: String,
    expiration: DateTime<Utc>,
    usage_count: i64,
}

fn generate_short_id(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (0..length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub async fn create_url(
    State((pool, _)): State<(PgPool, CleanupState)>,
    Json(payload): Json<CreateUrl>,
) -> Result<Json<UrlResponse>, (StatusCode, String)> {
    let id = generate_short_id(7);
    let expiration = Utc::now() + Duration::hours(24);

    let result = sqlx::query_as::<_, UrlResponse>(
        "INSERT INTO links (id, target_url, expiration, usage_count) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, target_url, expiration, usage_count"
    )
    .bind(&id)
    .bind(&payload.target_url)
    .bind(expiration)
    .bind(0)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    Ok(Json(result))
}

pub async fn get_url(
    State((pool, _)): State<(PgPool, CleanupState)>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url = sqlx::query_as::<_, UrlResponse>(
        "SELECT id, target_url, expiration, usage_count 
         FROM links 
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    match url {
        Some(url) => {
            if Utc::now() > url.expiration {
                return Err((StatusCode::FORBIDDEN, "URL has expired".to_string()));
            }
            
            if url.usage_count >= 5000 {
                return Err((StatusCode::TOO_MANY_REQUESTS, "URL usage limit exceeded".to_string()));
            }

            // Update usage count
            sqlx::query(
                "UPDATE links 
                 SET usage_count = usage_count + 1 
                 WHERE id = $1"
            )
            .bind(&url.id)
            .execute(&pool)
            .await
            .map_err(|e| {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
            })?;

            Ok(Redirect::to(&url.target_url))
        },
        None => Err((StatusCode::NOT_FOUND, "URL not found".to_string())),
    }
}

// Cleanup state to track last cleanup time
#[derive(Clone)]
pub struct CleanupState {
    last_cleanup: Arc<Mutex<DateTime<Utc>>>,
}

impl CleanupState {
    pub fn new() -> Self {
        CleanupState {
            last_cleanup: Arc::new(Mutex::new(Utc::now())),
        }
    }
}

pub async fn cleanup_expired_links(
    State((pool, cleanup_state)): State<(PgPool, CleanupState)>,
) -> Result<Json<String>, (StatusCode, String)> {
    const BATCH_SIZE: i64 = 1000;
    const MIN_CLEANUP_INTERVAL: Duration = Duration::minutes(1);

    // Check if enough time has passed since last cleanup
    let mut last_cleanup = cleanup_state.last_cleanup.lock().await;
    let now = Utc::now();
    
    if now - *last_cleanup < MIN_CLEANUP_INTERVAL {
        return Ok(Json("Cleanup skipped - too soon since last cleanup".to_string()));
    }
    *last_cleanup = now;

    let mut total_deleted = 0;
    loop {
        let result = sqlx::query(
            "WITH expired AS (
                SELECT id FROM links 
                WHERE expiration < $1 
                LIMIT $2
                FOR UPDATE
            )
            DELETE FROM links 
            WHERE id IN (SELECT id FROM expired)
            RETURNING id"
        )
        .bind(now)
        .bind(BATCH_SIZE)
        .execute(&pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
        })?;

        let deleted_count = result.rows_affected() as i64;
        total_deleted += deleted_count;

        if deleted_count < BATCH_SIZE {
            break;
        }

        time::sleep(time::Duration::from_millis(100)).await;
    }

    Ok(Json(format!("Cleanup completed: {} links deleted", total_deleted)))
}

pub async fn start_cleanup_task(
    pool: PgPool,
    cleanup_state: CleanupState,
) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(300));

        loop {
            interval.tick().await;
            match cleanup_expired_links(State((pool.clone(), cleanup_state.clone()))).await {
                Ok(Json(msg)) => println!("Cleanup task: {}", msg),
                Err((_, e)) => eprintln!("Cleanup task error: {}", e),
            }
        }
    });
}