use crate::UserRepoDto;

//--DTO-----//
pub struct UserUseCaseDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
impl From<UserRepoDto> for UserUseCaseDto {
    fn from(u: UserRepoDto) -> Self {
        Self {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            password: u.password_hash,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("not fond")]
    NotFond,
    #[error("error db {0}")]
    Db(String),
}
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("invalid")]
    Invalid,
    #[error("error db {0}")]
    Db(String),
}
#[async_trait::async_trait]
pub trait UserUseCase: Send + Sync {
    async fn user_login(&self, username: String, password: String) -> Result<(), AuthError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), ServiceError>;
    async fn get_users(&self) -> Result<Vec<UserUseCaseDto>, ServiceError>;
    async fn get_user(&self, id: String) -> Result<UserUseCaseDto, ServiceError>;
}
