use model::{
    auth_model::{UserLogin, UserSingup},
    jwt_key_model::jwt::TokenResponse,
};
use thiserror::Error;

#[async_trait::async_trait]
pub trait AuthUserCase: Send + Sync {
    async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError>;
    async fn singup(&self, order: UserSingup) -> Result<(), AuthUserCaseError>;
    async fn logout(&self, order: TokenResponse) -> Result<LogoutResult, AuthUserCaseError>;
    async fn refresh_token(&self, order: TokenResponse)
    -> Result<TokenResponse, AuthUserCaseError>;
}
//---dto----//
#[derive(Debug, PartialEq, Eq)]
pub enum LogoutResult {
    SessionTerminated,
    SessionNotFond,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthUserCaseError {
    #[error("auth repo error:{0}")]
    AuthRepoFail(String),
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
    #[error("Policyrepo fail repo error:{0}")]
    PolicyRepoError(String),
    #[error("BadRequet Requet")]
    BadRequet,
    #[error("An Authentication")]
    Authentication,
    #[error("Corrupted data")]
    Corrupted,
    #[error("PolicyVersion Mismatch")]
    PolicyVersionMismatch,
    #[error("Session Notfond")]
    SessionNotFond,
    #[error("RefreshExpired")]
    RefreshExpired,
}
