use model::{
    auth_model::{UserLogin, UserSingup},
    jwt_key_model::jwt::TokenResponse,
};
use thiserror::Error;

#[async_trait::async_trait]
pub trait AuthUserCase: Send + Sync {
    async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError>;
    async fn singup(&self, order: UserSingup) -> Result<(), AuthUserCaseError>;
    async fn logout(&self);
    async fn refresh(&self, order: TokenResponse) -> Result<TokenResponse, AuthUserCaseError>;
}

#[derive(Debug, Error)]
pub enum AuthUserCaseError {
    #[error("cashing repo error")]
    CashingFail(String),
    #[error("hasher repo error")]
    HashingFail(String),
    #[error("model error")]
    ModelFail(String),
    #[error("database repo error")]
    DbFail(String),
    #[error("Invalid Requet")]
    InvalidRequet,
}
