use crate::config::{Config, ConfigLoader};
use dotenvy::dotenv;
use std::env;

pub struct DotenvLoader;

impl ConfigLoader for DotenvLoader {
    fn load(&self) -> Result<Config, Box<dyn std::error::Error>> {
        dotenv().ok();
        Ok(Config {
            host: env::var("API_HOST")?,
            port: env::var("API_PORT")?.parse()?,
        })
    }
}
