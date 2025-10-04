use uuid::Uuid;

//-----user_DTO------//
#[derive(sqlx::FromRow, Debug)]
pub struct UserRepoDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
#[derive(sqlx::FromRow, Debug)]
pub struct UserAuthRepoDto {
    pub username: String,
    pub password_hash: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("db error {0}")]
    Db(String),
}
#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserRepoDto>, RepoError>;
    async fn new_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<Option<UserRepoDto>, RepoError>;
    async fn get_users(&self) -> Result<Option<Vec<UserRepoDto>>, RepoError>;
    async fn get_password_by_username(
        &self,
        username: String,
    ) -> Result<Option<UserAuthRepoDto>, RepoError>;
}
