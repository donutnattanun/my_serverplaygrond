use dotenvy::dotenv;
use std::env;
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct EnvConfig {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub jwt_kid: String,
    pub jwt_private_path: String,
    pub jwt_pubic_path: String,
}
#[derive(thiserror::Error, Debug)]
pub enum LoadEnvError {
    #[error("Env NotFond")]
    NotFond(String),
    #[error("Env var error")]
    Varfail,
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
    info!(env_jwt_kid=%env_jwt_kid,"load_env:load_env sucsess");
    let env_jwt_pubic_path = env::var("JWT_PUBIC_PATH").map_err(|e| {
        error!(error=%e,"load_env:load_env std::var");
        LoadEnvError::Varfail
    })?;
    info!(env_jwt_kid=%env_jwt_kid,"load_env:load_env sucsess");
    Ok(EnvConfig {
        host: env_host,
        port: env_port,
        database_url: env_database_url,
        jwt_kid: env_jwt_kid,
        jwt_private_path: env_jwt_pri_path,
        jwt_pubic_path: env_jwt_pubic_path,
    })
}
