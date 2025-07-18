use axum::{routing::{get,post},http::StatusCode,Json,Router,response::IntoResponse};
use serde::{Deserialize,Serialize};

#[derive(Deserialize)]
struct EchoUser{
    username:String,
}

#[derive(Serialize)]
struct Echo {
    id :u32,
    username : String,
} 
#[tokio::main]
async fn main(){
    let app =Router::new().route("/hello", post(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
async fn hello(Json(paylord): Json<EchoUser>)->(StatusCode,Json<Echo>){
    let echo = Echo{
        id : 1234,
        username : paylord.username,
        
    };
    (StatusCode::CREATED, Json(echo))
    
} 
