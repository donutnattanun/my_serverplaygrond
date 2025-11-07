use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use deadpool_redis::redis::AsyncCommands;
use model::{
    auth_model::{AuthFormatError, UserLogin, UserSingup},
    jwt::TokenResponse,
};
use std::sync::Arc;
use tracing::{error, info, warn};
use use_case::{AuthUserCase, AuthUserCaseError};

use crate::{
    dto::{UserLoginReq, UserSingupReq},
    err_map::ErrorToHttp,
};
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct SharedState {
    pub svc: Arc<dyn AuthUserCase + Send + Sync + 'static>,
    pub pg_pool: PgPool,
    pub redis_pool: deadpool_redis::Pool,
}

type AppState = SharedState;

pub fn routes(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .allow_credentials(true);
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/singup", post(singup))
        .route("/auth/logout", post(logout))
        .route("/auth/refresh", post(refresh))
        .route("/check", get(check))
        .layer(cors)
        .with_state(state)
}

pub async fn login(
    State(state): State<AppState>,
    Json(raw): Json<UserLoginReq>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let order: UserLogin = raw.try_into().map_err(|e: AuthFormatError| {
        warn!(error=%e,"Format error :");
        e.to_http()
    })?;
    let token = state
        .svc
        .login(order)
        .await
        .map_err(|e: AuthUserCaseError| {
            warn!(error=%e,"App error : ");
            e.to_http()
        })?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
        "status": "success",
        "message": "Login successful",
        "data": {
            "access_token": token.access_token,
            "refresh_token": token.refresh_token,
            "expires_in": token.expires_in,
            "token_type": token.token_type,
        }

        })),
    ))
}
pub async fn singup(
    State(state): State<AppState>,
    Json(raw): Json<UserSingupReq>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let order: UserSingup = raw.try_into().map_err(|e: AuthFormatError| {
        warn!(error=%e,"Format error :");
        e.to_http()
    })?;
    state
        .svc
        .singup(order)
        .await
        .map_err(|e: AuthUserCaseError| {
            warn!(error=%e,"App error : ");
            e.to_http()
        })?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
        "status": "success",
        "message": "singup successful",
        })),
    ))
}
pub async fn logout(
    State(state): State<AppState>,
    Json(raw): Json<TokenResponse>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    state
        .svc
        .logout(raw)
        .await
        .map_err(|e: AuthUserCaseError| {
            warn!(error=%e,"App error : ");
            e.to_http()
        })?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
        "status": "success",
        "message": "logout successful",
        })),
    ))
}
pub async fn refresh(
    State(state): State<AppState>,
    Json(raw): Json<TokenResponse>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    state
        .svc
        .refresh_token(raw)
        .await
        .map_err(|e: AuthUserCaseError| {
            warn!(error=%e,"App error : ");
            e.to_http()
        })?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
        "status": "success",
        "message": "refresh successful",
        })),
    ))
}

//TODO make tarit app
//now let manual
use serde_json::json;
use sqlx::PgPool;

pub async fn check(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    let pool = state.pg_pool;
    let db_ok = sqlx::query("SELECT 1").fetch_one(&pool).await.is_ok();
    let redis_ok = {
        if let Ok(mut conn) = state.redis_pool.get().await {
            conn.ping::<String>().await.is_ok()
        } else {
            false
        }
    };
    let ok = db_ok && redis_ok;

    let status = if ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    let time = chrono::Utc::now().to_string();

    (
        status,
        Json(json!({
            "status": if ok { "ok" } else { "degraded" },
            "services": {
                "database": db_ok,
                "redis": redis_ok,
            },
            "timestamp": time
        })),
    )
}
