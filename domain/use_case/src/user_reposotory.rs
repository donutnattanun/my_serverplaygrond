use async_trait::async_trait;
use model::users;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("db error {0}")]
    Db(String),
}
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<users>, RepoError>;
    async fn new_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<Option<users>, RepoError>;
    async fn get_users(&self) -> Result<Option<users>, RepoError>;
    async fn get_password_by_username(&self, username: String) -> Result<Option<users>, RepoError>;
}
