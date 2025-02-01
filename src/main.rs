mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection
    let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    // Build our application with routes
    let app = Router::new()
        .route("/url", post(routes::create_url))
        .route("/url/:id", get(routes::get_url))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

// routes.rs
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

#[derive(Serialize)]
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
    
    let result = sqlx::query_as!(
        UrlResponse,
        "INSERT INTO links (id, target_url) VALUES ($1, $2) RETURNING id, target_url",
        id,
        payload.target_url
    )
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
    let result = sqlx::query_as!(
        UrlResponse,
        "SELECT id, target_url FROM links WHERE id = $1",
        id
    )
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