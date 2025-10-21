use model::{auth_model::PasswordHash, users::Users};
use thiserror::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_password_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PasswordHash>, UserRepoError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<Users>, UserRepoError>;
    async fn creat_user(&self, user: Users) -> Result<(), UserRepoError>;
    async fn list_user(&self) -> Result<Vec<Users>, UserRepoError>;
}
#[derive(Debug, Error)]
pub enum UserRepoError {
    #[error("Engin error")]
    EnginError(String),
}
