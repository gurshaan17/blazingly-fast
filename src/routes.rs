use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
use base64::{Engine, engine::general_purpose};
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

fn generate_short_id() -> String {
    let random_number = rand::thread_rng().gen_range(0..u32::MAX);
    general_purpose::URL_SAFE_NO_PAD.encode(random_number.to_string())
}

pub async fn create_url(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUrl>,
) -> Result<Json<UrlResponse>, (StatusCode, String)> {
    let id = generate_short_id();
    
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
) -> Result<Json<UrlResponse>, (StatusCode, String)> {
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
        Some(url) => Ok(Json(url)),
        None => Err((StatusCode::NOT_FOUND, "URL not found".to_string())),
    }
}