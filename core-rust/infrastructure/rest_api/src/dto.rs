use model::{
    auth_model::{AuthFormatError, PasswordPlain, UserLogin, UserSingup},
    jwt::TokenResponse,
    users::{AccountStatus, Role},
};
use serde::Deserialize;
use uuid::Uuid;
#[derive(Debug, Deserialize)]
pub struct UserLoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSingupReq {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl TryFrom<UserSingupReq> for UserSingup {
    type Error = AuthFormatError;
    fn try_from(r: UserSingupReq) -> Result<Self, Self::Error> {
        let php = PasswordPlain::form_vec(r.password.as_bytes().to_vec());
        let res = UserSingup::new(r.username, r.email, php)?;
        Ok(res)
    }
}

impl TryFrom<UserLoginReq> for UserLogin {
    type Error = AuthFormatError;
    fn try_from(r: UserLoginReq) -> Result<Self, Self::Error> {
        let php = PasswordPlain::form_vec(r.password.as_bytes().to_vec());
        let res = UserLogin::new(r.username, php)?;
        Ok(res)
    }
}
#[derive(Debug, Deserialize)]
pub struct MasterUpdateReq {
    pub token: TokenResponse,
    pub user_id: Uuid,
    pub user_role: Role,
    pub user_status: AccountStatus,
}
