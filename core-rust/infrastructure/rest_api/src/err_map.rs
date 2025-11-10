use axum::{Json, http::StatusCode};
use model::auth_model::AuthFormatError;
use serde_json::json;
use use_case::{AuthUserCaseError, MasterUseCaseError};

pub trait ErrorToHttp {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>);
}
impl ErrorToHttp for AuthFormatError {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>) {
        match self {
            AuthFormatError::EmailError(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "bad request" })),
            ),
            AuthFormatError::UsernameError(_) => (
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
                Json(json!({ "status": "fail",
                "code": "invalid_token",
                "message": "The provided token is invalid or expired."})),
            ),

            SessionNotFond => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "session not found" })),
            ),

            PolicyVersionMismatch => (
                StatusCode::CONFLICT,
                Json(json!({ "status": "fail",
                "code": "policy_version_mismatch",
                "message": "Your session is no longer valid. Please log in again."})),
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

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "unknown error" })),
            ),
        }
    }
}
impl ErrorToHttp for MasterUseCaseError {
    fn to_http(&self) -> (StatusCode, Json<serde_json::Value>) {
        match self {
            MasterUseCaseError::JwtFail(e)
            | MasterUseCaseError::AuthRepoFail(e)
            | MasterUseCaseError::UserRepoFail(e)
            | MasterUseCaseError::PolicyRepoError(e)
            | MasterUseCaseError::HashingFail(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "error": e,
                })),
            ),

            MasterUseCaseError::SessionNotFond => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": "Session not found",
                })),
            ),

            MasterUseCaseError::RefreshExpired => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": "Refresh token expired",
                })),
            ),

            MasterUseCaseError::PolicyVersionMismatch => (
                StatusCode::CONFLICT,
                Json(json!({
                    "status": "error",
                    "message": "Policy version mismatch",
                })),
            ),

            MasterUseCaseError::BadRequet => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Bad request",
                })),
            ),

            MasterUseCaseError::PermittedFail => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "status": "error",
                    "message": "Operation not permitted",
                })),
            ),
        }
    }
}
