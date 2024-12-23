
use std::sync::Arc;
use axum::{
    response::IntoResponse, 
    routing::get, 
    Json, 
    Router
};
use tokio::net::TcpListener;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/api/healthcheck", 
        get(health_check_handler)
    );
    // log 
    println!("Server started successfully at 0.0.0.0:8080");

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


pub async fn health_check_handler() -> impl IntoResponse{
    const MESSAGE: &str = "API Services";
    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });
    Json(json_response)
}