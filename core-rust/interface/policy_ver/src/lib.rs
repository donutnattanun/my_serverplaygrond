use async_trait::async_trait;
use use_case::{CashPolicyInMemoty, PolicyRepo, PolicyRepoError};

#[async_trait]
impl PolicyRepo for CashPolicyInMemoty {
    async fn get_policy_version(&self) -> Result<i32, PolicyRepoError> {
        self.policy_ver
            .read()
            .map(|v| *v)
            .map_err(|e| PolicyRepoError::RwLockError(e.to_string()))
    }
    async fn bump_policy_version(&self) -> Result<i32, PolicyRepoError> {
        let mut ver = self
            .policy_ver
            .write()
            .map_err(|e| PolicyRepoError::RwLockError(e.to_string()))?;
        *ver += 1;
        Ok(*ver)
    }
}
