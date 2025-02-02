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
            let index = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect()
}

pub async fn create_url(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUrl>,
) -> Result<Json<UrlResponse>, (StatusCode, String)> {
    let id = generate_short_id(7);
    let expiration = Utc::now() + Duration::days(7);

    let result = sqlx::query_as::<_, UrlResponse>(
        "INSERT INTO links (id, target_url, expiration, usage_count) VALUES ($1, $2, $3, $4) RETURNING id, target_url, expiration, usage_count"
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
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as::<_, UrlResponse>(
        "SELECT id, target_url, expiration, usage_count FROM links WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    match result {
        Some(mut url) => {
            if Utc::now() > url.expiration {
                return Err((StatusCode::NOT_FOUND, "URL has expired".to_string()));
            }
            if url.usage_count >= 5000 {
                return Err((StatusCode::NOT_FOUND, "URL usage limit exceeded".to_string()));
            }

            url.usage_count += 1;

            sqlx::query("UPDATE links SET usage_count = $1 WHERE id = $2")
                .bind(url.usage_count)
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

pub async fn cleanup_expired_links(State(pool): State<PgPool>) {
    let _ = sqlx::query("DELETE FROM links WHERE expiration < $1")
        .bind(Utc::now())
        .execute(&pool)
        .await;
}