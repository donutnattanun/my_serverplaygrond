use async_trait::async_trait;
use model::{
    auth_model::PasswordHash,
    users::{AcconutStatus, Role, Users},
};
use thiserror::Error;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_password_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PasswordHash>, UserRepoError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<Users>, UserRepoError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Users, UserRepoError>;
    async fn creat_user(
        &self,
        username: &str,
        email: &str,
        passwordhash: PasswordHash,
    ) -> Result<(), UserRepoError>;
    async fn list_user(&self) -> Result<Vec<Users>, UserRepoError>;
    async fn check_username(&self, username: &str) -> Result<Option<()>, UserRepoError>;
    async fn check_email(&self, email: &str) -> Result<Option<()>, UserRepoError>;
    async fn update_user_status_role(
        &self,
        user_id: Uuid,
        user_status: AcconutStatus,
        user_role: Role,
    ) -> Result<(), UserRepoError>;
}
#[derive(Debug, Error)]
pub enum UserRepoError {
    #[error("Engin error")]
    EnginError(String),
}
