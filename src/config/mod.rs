use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    match dotenv() {
        Ok(path) => {
            println!(".env loader from {:?}", path);
        }
        Err(e) => {
            println!(" DontenvLoader error={e:?}");
            return Err(e.into());
        }
    };
    let host = env::var("API_HOST").map_err(|_| "Missing API_HOST")?;
    let port = env::var("API_PORT").map_err(|_| "Missing API_PORT")?;
    let database_url = env::var("DATABASE_URL").map_err(|_| "Missing DATABASE_URL")?;
    Ok(Config {
        host: host,
        port: port,
        database_url: database_url,
    })
}
