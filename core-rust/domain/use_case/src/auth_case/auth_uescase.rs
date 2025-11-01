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
    #[error("cashing repo error:{0}")]
    CashingFail(String),
    #[error("hasher repo error:{0}")]
    HashingFail(String),
    #[error("jwt repo error:{0}")]
    JwtRepofail(String),
    #[error("model error:{0}")]
    ModelFail(String),
    #[error("database repo error:{0}")]
    DbFail(String),
    #[error("refresh repo error:{0}")]
    RefechFail(String),
    #[error("BadRequet Requet")]
    BadRequet,
    #[error("An Authentication")]
    Authentication,
    #[error("Corrupted data")]
    Corrupted,
}
