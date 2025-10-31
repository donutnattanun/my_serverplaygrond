use model::auth_model::{PasswordHash, PasswordPlain};
use thiserror::Error;

#[async_trait::async_trait]
pub trait HashRepo: Send + Sync {
    async fn hashing_password_argon2(
        &self,
        ps_plain: PasswordPlain,
    ) -> Result<PasswordHash, HasherError>;
    async fn varify_password_argon2(
        &self,
        phc: PasswordHash,
        cadidaie: PasswordPlain,
    ) -> Result<VerifyStatus, HasherError>;
    async fn hash_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &String,
    ) -> Result<String, HasherError>;
    async fn varify_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &String,
        rt_hash_bash64: &String,
    ) -> Result<VerifyStatus, HasherError>;
}

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("engin hash error")]
    EnginError(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStatus {
    Pass,
    Fail,
    Corrupted,
}
