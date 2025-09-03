use axum::{Router, http::StatusCode, routing::get};
use tokio;

#[tokio::main()]
async fn main() {
    let app = Router::new().route("/check", get(check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "am ok hell ya")
}
