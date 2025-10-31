use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait RefreshRepo: Send + Sync {
    async fn gen_refresh_token_base64(
        &self,
        now: i64,
        rt_ttl: i64,
    ) -> Result<RefreshToken, RefreshRepoError>;
}

#[derive(Debug, Error)]
pub enum RefreshRepoError {
    #[error("Engin Error")]
    Enginfail(String),
}
//dto
pub struct RefreshToken {
    pub token_plain: String,
    pub token_exp: i64,
}
impl RefreshToken {
    pub fn new(token_plain: String, token_exp: i64) -> Self {
        Self {
            token_plain,
            token_exp,
        }
    }
}
