use postgre_db::SqlxUserRepo;
use service::UserService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use use_case::UserUseCase;
type AppState = Arc<dyn UserUseCase + Send + Sync>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("statrapp");
    let env = load_env::load().unwrap();
    let db_url = env.database_url;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    let repo = Arc::new(SqlxUserRepo::new(pool));
    let svc = Arc::new(UserService::new(repo));
    let app = rest_api::routes(svc);
    let addr_api = format!("{}:{}", env.host, env.port);

    println!("app runing at {:?}", &addr_api);
    let listener = tokio::net::TcpListener::bind(&addr_api).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
