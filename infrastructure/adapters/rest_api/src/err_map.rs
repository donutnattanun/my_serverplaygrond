use axum::{Json, http::StatusCode};
use serde_json::{Value, json};
use use_case::{AuthError, ServiceError};

pub fn to_http(err: ServiceError) -> (StatusCode, Json<Value>) {
    match err {
        ServiceError::NotFond => (StatusCode::NOT_FOUND, Json(json!({"error":"not found"}))),
        ServiceError::Db(msg) => (StatusCode::BAD_REQUEST, Json(json!({"error":msg}))),
    }
}
pub fn auth_http(err: AuthError) -> (StatusCode, Json<Value>) {
    match err {
        AuthError::Invalid => (StatusCode::BAD_REQUEST, Json(json!({"error":"invalid"}))),
        AuthError::Db(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":msg})),
        ),
    }
}
