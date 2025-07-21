use crate::config::entity::Config;

pub trait ConfigLoader {
    fn load(&self) -> Result<Config, Box<dyn std::error::Error>>;
}
