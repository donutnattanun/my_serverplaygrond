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
        rt_plain_base64: &str,
    ) -> Result<String, HasherError>;
    async fn varify_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &str,
        rt_hash_bash64: &str,
    ) -> Result<VerifyStatus, HasherError>;
}

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("engin hash error:{0}")]
    EnginError(String),
    #[error("format error:{0}")]
    FormatError(String),
    #[error("HashEngin error:{0}")]
    HashEnginError(String),
    #[error{"varify_password error{0}"}]
    VerifyError(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStatus {
    Pass,
    Fail,
    Corrupted,
}
