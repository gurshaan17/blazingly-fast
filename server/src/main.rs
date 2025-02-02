mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use crate::routes::{CleanupState, start_cleanup_task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Initialize cleanup state
    let cleanup_state = CleanupState::new();
    
    // Start the cleanup task
    start_cleanup_task(pool.clone(), cleanup_state.clone()).await;

    // Create a router with shared state
    let app = Router::new()
        .route("/url", post(routes::create_url))
        .route("/url/:id", get(routes::get_url))
        .route("/cleanup", post(routes::cleanup_expired_links))
        .with_state((pool, cleanup_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Start the server using hyper
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}