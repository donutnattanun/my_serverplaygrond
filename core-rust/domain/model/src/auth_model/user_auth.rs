use crate::auth_model::PasswordHash;
//for login and singup//
#[derive(Debug)]
pub struct UserLoginRequest {
    pub username: String,
    pub password_hash: PasswordHash,
}

#[derive(Debug)]
pub struct UserSingup {
    pub username: String,
    pub email: String,
    pub password_hash: PasswordHash,
}
