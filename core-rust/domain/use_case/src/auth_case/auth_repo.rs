use model::jwt_key_model::jwt::{AuthConfig, SessionRecord, TokenResponse};
use thiserror::Error;
#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync {
    async fn get_sesion_by_at(&self, token: TokenResponse) -> Result<SessionRecord, AuthRepoError>;
    async fn refresh_rt(
        &self,
        cfg: AuthConfig,
        token: TokenResponse,
    ) -> Result<SessionRecord, AuthRepoError>;
    async fn creat_session(
        &self,
        cfg: AuthConfig,
        session: SessionRecord,
    ) -> Result<(), AuthRepoError>;
    async fn kill_sesion_id(&self, token: TokenResponse) -> Result<(), AuthRepoError>;
}

#[derive(Debug, Error)]
pub enum AuthRepoError {
    #[error("Engin error")]
    EnginFail(String),
}
