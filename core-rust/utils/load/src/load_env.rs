use dotenvy::dotenv;
use std::env;
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct EnvConfig {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_kid: String,
    pub jwt_private_path: String,
    pub jwt_pubic_path: String,
    pub hmac_sha256_key_path: String,
    pub at_ttl: u32,
    pub rt_ttl: u32,
    pub ss_ttl: u32,
}
#[derive(thiserror::Error, Debug)]
pub enum LoadEnvError {
    #[error("Env NotFond : {0}")]
    NotFond(String),
    #[error("Env var error")]
    Varfail,
    #[error("FormatEnv error : {0}")]
    Formaterror(String),
}

pub fn load_env() -> Result<EnvConfig, LoadEnvError> {
    match dotenv() {
        Ok(path) => {
            info!("dotenv load success use env at {:?}", path);
            var_env()
        }
        Err(e) => {
            warn!("fallback try_env form os error::{:?}", e);
            Err(LoadEnvError::NotFond(e.to_string()))
        }
    }
}
pub fn var_env() -> Result<EnvConfig, LoadEnvError> {
    //TODO let make it look better then
    let env_at_ttl = env::var("ACSECC_TOKEN_TTL").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    let env_at_ttl = env_at_ttl
        .trim()
        .parse::<u32>()
        .map_err(|e| LoadEnvError::Formaterror(e.to_string()))?;
    info!(env_at_ttl=%env_at_ttl,"load_env:load_env sucsess");

    let env_rt_ttl = env::var("REFRESH_TOKEN_TTL").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    let env_rt_ttl = env_rt_ttl
        .trim()
        .parse::<u32>()
        .map_err(|e| LoadEnvError::Formaterror(e.to_string()))?;
    info!(env_rt_ttl=%env_rt_ttl,"load_env:load_env sucsess");

    let env_ss_ttl = env::var("SESSION_TTL").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    let env_ss_ttl = env_ss_ttl
        .trim()
        .parse::<u32>()
        .map_err(|e| LoadEnvError::Formaterror(e.to_string()))?;
    info!(env_ss_ttl=%env_ss_ttl,"load_env:load_env sucsess");

    let env_hmac_sha256_key_path = env::var("HMAC_SHA256_KEY_PATH").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_hmac_sha256_key_path=%env_hmac_sha256_key_path,"load_env:load_env sucsess");

    let env_redis_url = env::var("REDIS_URL").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_redis_url=%env_redis_url,"load_env:load_env sucsess");

    let env_host = env::var("API_HOST").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_host=%env_host,"load_env:load_env sucsess");
    let env_port = env::var("API_PORT").map_err(|e| {
        error!(error=%e,"losd_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_port=%env_port,"load_env:load_env sucsess");

    let env_database_url = env::var("DATABASE_URL").map_err(|e| {
        error!(error=%e,"load_env:load_env error std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_database_url=%env_database_url,"load_env:load_env sucsess");
    let env_jwt_kid = env::var("JWT_KID").map_err(|e| {
        error!(error=%e,"load_env:load_env std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_jwt_kid=%env_jwt_kid,"load_env:load_env sucsess");
    let env_jwt_pri_path = env::var("JWT_PRIVATE_PATH").map_err(|e| {
        error!(error=%e,"load_env:load_env std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_jwt_pri_path=%env_jwt_pri_path,"load_env:load_env sucsess");
    let env_jwt_pubic_path = env::var("JWT_PUBLIC_PATH").map_err(|e| {
        error!(error=%e,"load_env:load_env std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_jwt_pubic_path=%env_jwt_pubic_path,"load_env:load_env sucsess");
    Ok(EnvConfig {
        host: env_host,
        port: env_port,
        database_url: env_database_url,
        redis_url: env_redis_url,
        hmac_sha256_key_path: env_hmac_sha256_key_path,
        jwt_kid: env_jwt_kid,
        jwt_private_path: env_jwt_pri_path,
        jwt_pubic_path: env_jwt_pubic_path,
        at_ttl: env_at_ttl,
        ss_ttl: env_ss_ttl,
        rt_ttl: env_rt_ttl,
    })
}
