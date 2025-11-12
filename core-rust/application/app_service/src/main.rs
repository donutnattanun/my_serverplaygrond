use cashing_redis::CashRedisService;
use hasher::HashService;
use jwt_token::{self, JwtCfg, JwtService};
use load::Security;
use model::{self, jwt::AuthConfig};
use policy_ver::{self, CashPolicyInMemoty};
use postgre_db::{self, SqlxUserRepo};
use refresh_token::{self, RefreshTokenService};
use rest_api::{SharedState, routes};
use service::{AuthService, MasterService};
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};
use timesystem::{self, TimeSystemService};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init_tracing()?;
    let env = load::load_env()?;
    //----DI state ----//
    let secret_key = Security::form_load_file(
        &env.jwt_kid,
        &env.jwt_private_path,
        &env.jwt_pubic_path,
        &env.hmac_sha256_key_path,
    )?;
    let auth_repo = Arc::new(CashRedisService::new(&env.redis_url)?);
    let hash_repo = Arc::new(HashService::new_default(secret_key.hmac_key));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&env.database_url)
        .await?;
    let user_repo = Arc::new(SqlxUserRepo::new(pool.clone()));
    let jwt_cfg = JwtCfg::new_option(&env.jwt_kid);
    let jwt_repo = Arc::new(JwtService::new(
        jwt_cfg,
        secret_key.dec_pem,
        secret_key.enc_pem,
    ));
    let time_repo = Arc::new(TimeSystemService::new());
    let rt_repo = Arc::new(RefreshTokenService::new());
    let policy_repo = Arc::new(CashPolicyInMemoty::new(1));
    let auth_cfg = AuthConfig::new(env.at_ttl, env.rt_ttl, env.ss_ttl);
    info!("Dependency Injection success");
    //----buid app state -----//
    let master_service = MasterService::new(
        user_repo.clone(),
        policy_repo.clone(),
        jwt_repo.clone(),
        auth_repo.clone(),
        hash_repo.clone(),
        time_repo.clone(),
    );

    let auth_service = AuthService::new(
        auth_repo.clone(),
        hash_repo,
        user_repo,
        jwt_repo,
        time_repo,
        rt_repo,
        policy_repo,
        auth_cfg,
    );
    let appstate = SharedState {
        svc: Arc::new(auth_service),
        svc_master: Arc::new(master_service),
        pg_pool: pool.clone(),
        redis_pool: auth_repo.redis_pool.clone(),
    };
    info!("Build app success");
    //-----buid router -----//
    let app = routes(appstate);
    let addr_api = format!("{}:{}", env.host, env.port);
    let listener = tokio::net::TcpListener::bind(&addr_api).await.unwrap();
    info!("app start success full");
    info!("app runing at {:?}", &addr_api);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
