use model::auth_model::{PasswordHash, PasswordPlain};
use thiserror::Error;

#[async_trait::async_trait]
pub trait HashRepo: Send + Sync {
    async fn hashing(&self, plain: PasswordPlain) -> Result<PasswordHash, HasherError>;
    async fn varify(&self, phc: PasswordHash, cadidaie: PasswordPlain) -> Result<(), HasherError>;
}

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("password model error")]
    PasswordModel(String),
    #[error("engin hash error")]
    EnginError,
}
