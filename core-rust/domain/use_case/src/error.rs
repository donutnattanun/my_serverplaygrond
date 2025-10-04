#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("invalid")]
    Invalid,
    #[error("error db {0}")]
    Db(String),
}
