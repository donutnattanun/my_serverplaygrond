use model::{jwt::Claims, jwt_key_model::jwt::SessionRecord};
use thiserror::Error;
#[async_trait::async_trait]
pub trait JwtRepo: Send + Sync {
    async fn encoder(
        &self,
        session: &SessionRecord,
        at_ttl: i64,
    ) -> Result<(String, i64), JwtRepoError>;
    async fn decoder(&self, token: &str) -> Result<Claims, JwtRepoError>;
}
#[derive(Debug, Error)]
pub enum JwtRepoError {
    #[error("Engin Error")]
    EnginFail(String),
}
