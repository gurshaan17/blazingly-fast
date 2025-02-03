use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use crate::routes::{CleanupState, start_cleanup_task};
use dotenv::dotenv;
use std::env;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenv().ok();
    
    // Retrieve the database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Connected to the database successfully.");
    
    // Initialize cleanup state
    let cleanup_state = CleanupState::new();
    
    // Start the cleanup task
    start_cleanup_task(pool.clone(), cleanup_state.clone()).await;

    // Create a router with shared state
    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/url", post(routes::create_url))
        .route("/url/:id", get(routes::get_url))
        .route("/cleanup", post(routes::cleanup_expired_links))
        .with_state((pool, cleanup_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);

    // Start the server using hyper
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
