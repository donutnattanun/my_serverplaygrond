use crate::config::{Config, ConfigLoader};
use dotenvy::dotenv;
use std::env;

pub struct DotenvLoader;

impl ConfigLoader for DotenvLoader {
    fn load(&self) -> Result<Config, Box<dyn std::error::Error>> {
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
        Ok(Config {
            host: host,
            port: port,
        })
    }
}
