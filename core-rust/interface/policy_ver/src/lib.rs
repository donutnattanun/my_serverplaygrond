use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use use_case::{PolicyRepo, PolicyRepoError};

#[derive(Debug)]
pub struct CashPolicyInMemoty {
    pub policy_ver: Arc<RwLock<u32>>,
}
impl CashPolicyInMemoty {
    pub fn new(initial: u32) -> Self {
        Self {
            policy_ver: Arc::new(RwLock::new(initial)),
        }
    }
}

#[async_trait]
impl PolicyRepo for CashPolicyInMemoty {
    async fn get_policy_version(&self) -> Result<u32, PolicyRepoError> {
        let value = self.policy_ver.read().await;
        Ok(*value)
    }
    async fn bump_policy_version(&self) -> Result<u32, PolicyRepoError> {
        let mut ver = self.policy_ver.write().await;
        *ver += 1;
        Ok(*ver)
    }
}
