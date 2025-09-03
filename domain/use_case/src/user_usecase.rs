use async_trait::async_trait;
use model::users;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("service NotFound")]
    NotFound,
    #[error("service error {0}")]
    ServiceError(String),
}
#[async_trait]
pub trait UserUescase: Send + Sync {
    async fn get_users(&self) -> Result<users, ServiceError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<users, ServiceError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<users, ServiceError>;
    async fn login_user(&self, username: String, password: String) -> Result<users, ServiceError>;
}
