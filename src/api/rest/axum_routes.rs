use axum::{
    Router,
    routing::{get, post},
};
use handlers;
pub fn routes() -> Router {
    Router::new()
        .route("/test_get", get(handle_testget))
        .route("/test_post", post(handle_testpost))
}
