// main.rs
mod routes;

use axum::{
    Router,
    routing::get,
};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection
    let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let pool = PgPool::connect(database_url).await?;
    
    // Build our application with routes
    let app = Router::new()
        .route("/status", get(routes::status_handler))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

// routes.rs
use axum::{
    response::Json,
    extract::State,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn status_handler(
    State(pool): State<PgPool>
) -> Json<Value> {
    // Test database connection
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => Json(json!({
            "status": "server working",
            "database": "connected"
        })),
        Err(e) => Json(json!({
            "status": "server working",
            "database": format!("error: {}", e)
        }))
    }
}