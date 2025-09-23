use tracing_subscriber::{EnvFilter, filter, fmt, prelude::*};
#[derive(thiserror::Error, Debug)]
pub enum LogErr {
    #[error("invalid LOG_FORMAT{0}")]
    InvalidFormat(String),
}

pub fn init_tracing() -> Result<(), LogErr> {
    let fmt_is_json = match std::env::var("LOG_FORMAT") {
        Ok(s) if s.eq_ignore_ascii_case("json") => true,
        Ok(s) if s.eq_ignore_ascii_case("default") || s.is_empty() => false,
        Err(e) => return Err(LogErr::InvalidFormat(e.to_string())),
        Ok(_) => false,
    };
    //let filter =EnvFilter::try_from_default_env().unwrap_or_else(op) ;
    if fmt_is_json {
        tracing_subscriber::registry()
            .with(fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().compact())
            .init();
    }
    Ok(())
}
