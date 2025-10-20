use model::Users;

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
    async fn create_user(&self, req: Users) -> Result<(), ServiceError>;
    async fn get_users(&self) -> Result<Vec<Users>, ServiceError>;
    async fn get_user(&self, id: String) -> Result<Users, ServiceError>;
}
//--validatetion--//
pub struct Valid<T>(pub T);

#[derive(Debug)]
pub struct UserLoginOrder {
    pub username: String,
    pub password: String,
}
#[derive(Debug)]
pub struct UserSingupOrder {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl Valid<UserSingupOrder> {
    pub fn new(username: String, email: String, password: String) -> Result<Self, AuthError> {
        //logic validation contrak
        if username.trim().is_empty() {
            return Err(AuthError::Invalid);
        }
        if password.len() < 4 {
            return Err(AuthError::Invalid);
        }
        if email.trim().is_empty() {
            return Err(AuthError::Invalid);
        }
        Ok(Valid(UserSingupOrder {
            username,
            email,
            password,
        }))
    }
}
impl Valid<UserLoginOrder> {
    pub fn new(username: String, password: String) -> Result<Self, AuthError> {
        //logic validation contrak
        if username.trim().is_empty() {
            return Err(AuthError::Invalid);
        }
        if password.len() < 4 {
            return Err(AuthError::Invalid);
        }
        Ok(Valid(UserLoginOrder { username, password }))
    }
}
//------------------//
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid_userloginorder_ok() {
        let got =
            Valid::<UserLoginOrder>::new("donut".into(), "1234".into()).expect("error fn Vlid<T>");
        assert_eq!(got.0.username, "donut");
        assert_eq!(got.0.password, "1234");
    }
    #[test]
    fn valid_userloginorder_error_case_is_empty() {
        let got = Valid::<UserLoginOrder>::new(" ".into(), "12345".into());
        assert!(matches!(got, Err(AuthError::Invalid)));
    }
    #[test]
    fn valid_userloginorder_error_case_len_less_4() {
        let got = Valid::<UserLoginOrder>::new("donut".into(), "123".into());
        assert!(matches!(got, Err(AuthError::Invalid)));
    }
}
