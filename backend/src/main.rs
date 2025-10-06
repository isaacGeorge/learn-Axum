use axum::{
    extract::Path,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Create a router with two routes
    let app = Router::new()
        .route("/", get(root))
        .route("/user/{id}", get(get_user))
        .route("/user/{user_id}/post/{post_id}", get(get_post));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("🚀 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// The root route handler
async fn root() -> &'static str {
    "Welcome to the Axum API!"
}

// The user route handler with path parameter
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

async fn get_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("User {} - Post {}", user_id, post_id)
}
