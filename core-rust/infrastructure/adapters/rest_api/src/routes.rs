use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use use_case::UserUseCase;

use crate::{UserLoginReq, UserReq, UserResp, err_map::auth_http, to_http};
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

type AppState = Arc<dyn UserUseCase + Send + Sync + 'static>;

pub fn routes(svc: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .allow_credentials(true);
    Router::new()
        .route("/users", get(list_users))
        .route("/user/{id}", get(get_user))
        .route("/newuser", post(newuser))
        .route("/user/login", post(login))
        .route("/check", get(check))
        .layer(cors)
        .with_state(svc)
}
#[axum::debug_handler]
pub async fn list_users(
    State(svc): State<AppState>,
) -> Result<Json<Vec<UserResp>>, (StatusCode, Json<serde_json::Value>)> {
    let users = svc.get_users().await.map_err(to_http)?;
    let resp = users.into_iter().map(Into::into).collect();
    Ok(Json(resp))
}
#[axum::debug_handler]

pub async fn get_user(
    State(svc): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<UserResp>, (StatusCode, Json<serde_json::Value>)> {
    let row = svc.get_user(id).await.map_err(to_http)?;
    Ok(Json(row.into()))
}
#[axum::debug_handler]

pub async fn newuser(
    State(svc): State<AppState>,
    Json(req): Json<UserReq>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    svc.create_user(req.username, req.email, req.password)
        .await
        .map_err(to_http)?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status":"Succassfull"})),
    ))
}
#[axum::debug_handler]

pub async fn login(
    State(svc): State<AppState>,
    Json(raw): Json<UserLoginReq>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let valid_order = raw.try_into().map_err(auth_http)?;
    svc.user_login(valid_order).await.map_err(auth_http)?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"status":"login Succsssfull",
                                "jwt":"1234"})),
    ))
}

pub async fn check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "am ok hell ya")
}
