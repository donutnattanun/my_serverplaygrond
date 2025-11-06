use axum::{Json, http::StatusCode};
use model::auth_model::AuthFormatError;
use serde_json::json;
use use_case::AuthUserCaseError;

pub trait ErrorToHttp {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>);
}
impl ErrorToHttp for AuthFormatError {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>) {
        match self {
            AuthFormatError::EmailError => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "bad request" })),
            ),
            AuthFormatError::UsernameError => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "bad request" })),
            ),
        }
    }
}

impl ErrorToHttp for AuthUserCaseError {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>) {
        use AuthUserCaseError::*;

        match self {
            BadRequet => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "bad request" })),
            ),

            Authentication => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "authentication failed" })),
            ),

            RefreshExpired => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "refresh token expired" })),
            ),

            SessionNotFond => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "session not found" })),
            ),

            PolicyVersionMismatch => (
                StatusCode::CONFLICT,
                Json(json!({ "error": "policy version mismatch" })),
            ),

            Corrupted => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "corrupted data" })),
            ),

            AuthRepoFail(msg) | HashingFail(msg) | JwtRepofail(msg) | ModelFail(msg)
            | DbFail(msg) | RefechFail(msg) | PolicyRepoError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            ),

            // เผื่อ case อื่น ๆ ในอนาคต
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "unknown error" })),
            ),
        }
    }
}
