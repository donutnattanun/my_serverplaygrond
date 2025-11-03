use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

use async_trait::async_trait;

#[derive(Debug)]
pub struct CashPolicyInMemoty {
    pub policy_ver: Arc<RwLock<i32>>,
}
impl CashPolicyInMemoty {
    pub fn new(initial: i32) -> Self {
        Self {
            policy_ver: Arc::new(RwLock::new(initial)),
        }
    }
}

#[async_trait]
pub trait PolicyRepo: Send + Sync {
    async fn get_policy_version(&self) -> Result<i32, PolicyRepoError>;
    async fn bump_policy_version(&self) -> Result<i32, PolicyRepoError>;
}

#[derive(Debug, Error)]
pub enum PolicyRepoError {
    #[error("RwLock poisoned")]
    RwLockError(String),
}
