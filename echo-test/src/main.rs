use axum::{Json, Router, routing::post};
use serde_json::Value;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Define the route
    let app = Router::new().route("/api/echo", post(echo_handler));

    // 2. Define the address (0.0.0.0:9080)
    let addr = SocketAddr::from(([0, 0, 0, 0], 9080));
    println!("Server listening on http://{}", addr);

    // 3. Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// The handler extracts the JSON body as a serde_json::Value and returns it
async fn echo_handler(Json(payload): Json<Value>) -> Json<Value> {
    Json(payload)
}
