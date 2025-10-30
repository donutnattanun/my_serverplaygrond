use model::jwt_key_model::jwt::SessionRecord;
use thiserror::Error;
#[async_trait::async_trait]
pub trait JwtRepo: Send + Sync {
    async fn encoder(&self, session: &SessionRecord) -> Result<String, JwtRepoError>;
    async fn decoder(&self, token: String) -> Result<SessionRecord, JwtRepoError>;
}
#[derive(Debug, Error)]
pub enum JwtRepoError {
    #[error("Engin Error")]
    EnginFail(String),
}
