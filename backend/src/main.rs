use axum::{routing::get, Router, Json};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Backend running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Backend running 🚀"
}

async fn hello() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello from Rust backend!" }))
}
