use axum::{
    extract::{Path, State},
    response::{Json, Redirect},
    http::StatusCode,
    response::IntoResponse,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUrl {
    target_url: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UrlResponse {
    id: String,
    target_url: String,
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
    
    let result = sqlx::query_as::<_, UrlResponse>(
        "INSERT INTO links (id, target_url) VALUES ($1, $2) RETURNING id, target_url"
    )
    .bind(&id)
    .bind(&payload.target_url)
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
        "SELECT id, target_url FROM links WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    match result {
        Some(url) => Ok(Redirect::to(&url.target_url)),
        None => Err((StatusCode::NOT_FOUND, "URL not found".to_string())),
    }
}