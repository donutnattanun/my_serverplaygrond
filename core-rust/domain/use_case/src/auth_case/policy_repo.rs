use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait PolicyRepo: Send + Sync {
    async fn get_policy_version(&self) -> Result<u32, PolicyRepoError>;
    async fn bump_policy_version(&self) -> Result<u32, PolicyRepoError>;
}

#[derive(Debug, Error)]
pub enum PolicyRepoError {
    #[error("RwLock poisoned")]
    RwLockError(String),
}
