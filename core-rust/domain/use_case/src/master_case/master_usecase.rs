use async_trait::async_trait;
use model::{
    jwt::TokenResponse,
    users::{AccountStatus, Role},
};
use thiserror::Error;
use uuid::Uuid;

#[async_trait]
pub trait MasterUseCase: Send + Sync {
    async fn update_user_status(
        &self,
        order: TokenResponse,
        user_id: Uuid,
        role: Role,
        status: AccountStatus,
    ) -> Result<MasterRespon, MasterUseCaseError>;
}
#[derive(Debug)]
pub enum MasterRespon {
    Update { new_policy_ver: u32 },
    Noop,
}

#[derive(Debug, Error)]
pub enum MasterUseCaseError {
    #[error("JwtRepoError error:{0}")]
    JwtFail(String),
    #[error("AuthRepoError error:{0}")]
    AuthRepoFail(String),
    #[error("UserRepo error:{0}")]
    UserRepoFail(String),
    #[error("SessionNotFond ")]
    SessionNotFond,
    #[error("HashingFail error:{0}")]
    HashingFail(String),
    #[error("MasterRefreshExpired")]
    RefreshExpired,
    #[error("PolicyRepoError error:{0}")]
    PolicyRepoError(String),
    #[error("MasterPolicyVersionMismatch")]
    PolicyVersionMismatch,
    #[error("MasterBadRequet")]
    BadRequet,
    #[error("Operation not permitted")]
    PermittedFail,
}
