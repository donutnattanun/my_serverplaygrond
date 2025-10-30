use model::jwt_key_model::jwt::SessionRecord;
use thiserror::Error;
#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync {
    async fn get_sesion_by_sess_id(
        &self,
        session_id: String,
    ) -> Result<SessionRecord, AuthRepoError>;
    async fn create_session(&self, session: SessionRecord) -> Result<(), AuthRepoError>;
    async fn kill_sesion_id(&self, session_id: String) -> Result<(), AuthRepoError>;
}

#[derive(Debug, Error)]
pub enum AuthRepoError {
    #[error("Engin error")]
    EnginFail(String),
}
