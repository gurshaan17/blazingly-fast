use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode,
};
use std::net::SocketAddr;
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/status", get(status_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn status_handler() -> Json<Value> {
    Json(json!({
        "status": "server working"
    }))
}