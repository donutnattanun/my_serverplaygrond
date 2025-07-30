use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
mod config;

#[derive(Deserialize)]
struct EchoUser {
    username: String,
}

#[derive(Serialize)]
struct Echo {
    id: u32,
    username: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //cell tait
    let config = config::load()?;
    let addr = format!("{}:{}", &config.host, &config.port);
    let app = Router::new().route("/hello", post(hello));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("app listener at {:?}", &addr);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
async fn hello(Json(paylord): Json<EchoUser>) -> (StatusCode, Json<Echo>) {
    let echo = Echo {
        id: 1234,
        username: paylord.username,
    };
    (StatusCode::CREATED, Json(echo))
}
