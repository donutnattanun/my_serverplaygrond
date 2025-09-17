use crate::AuthError;
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
//--validatetion--//
pub struct Valid<T>(pub T);

#[derive(Debug)]
pub struct UserLoginOrder {
    pub username: String,
    pub password: String,
}
#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("not fond")]
    NotFond,
    #[error("error db {0}")]
    Db(String),
}

#[async_trait::async_trait]
pub trait UserUseCase: Send + Sync {
    async fn user_login(&self, req: Valid<UserLoginOrder>) -> Result<(), AuthError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), ServiceError>;
    async fn get_users(&self) -> Result<Vec<UserUseCaseDto>, ServiceError>;
    async fn get_user(&self, id: String) -> Result<UserUseCaseDto, ServiceError>;
}
