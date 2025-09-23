use dotenvy::dotenv;
use std::env;
use tracing::{error, info};

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
}
#[derive(thiserror::Error, Debug)]
pub enum LoadEnvError {
    #[error("Env NotFond")]
    NotFond,
    #[error("Env var error")]
    Varfail,
}

pub fn load() -> Result<Config, LoadEnvError> {
    match dotenv() {
        Ok(path) => {
            info!(path=?path,"load_env:load from");
        }
        Err(e) => {
            println!(" DontenvLoader error={e:?}");
            error!(error=%e,"load_env:load error start");
            return Err(LoadEnvError::NotFond);
        }
    };
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

    Ok(Config {
        host: env_host,
        port: env_port,
        database_url: env_database_url,
    })
}
