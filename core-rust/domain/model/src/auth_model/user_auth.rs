use crate::auth_model::PasswordPlain;
use thiserror::Error;
//for login and singup//
#[derive(Debug)]
pub struct UserLogin {
    pub username: String,
    pub password_plain: PasswordPlain,
}

#[derive(Debug)]
pub struct UserSingup {
    pub username: String,
    pub email: String,
    pub password_plain: PasswordPlain,
}
impl UserLogin {
    pub fn new(username: String, password_plain: PasswordPlain) -> Result<Self, AuthFormatError> {
        if username.is_empty() || username.len() < 4 {
            return Err(AuthFormatError::UsernameError(username.to_string()));
        }
        Ok(Self {
            username,
            password_plain,
        })
    }
}

impl UserSingup {
    pub fn new(
        username: String,
        email: String,
        password_plain: PasswordPlain,
    ) -> Result<Self, AuthFormatError> {
        if username.is_empty() || username.len() < 4 {
            return Err(AuthFormatError::UsernameError(username.to_string()));
        }
        if email.is_empty() || !email.contains("@") {
            return Err(AuthFormatError::EmailError(email.to_string()));
        }
        Ok(Self {
            username,
            email,
            password_plain,
        })
    }
}
#[derive(Debug, Error)]
pub enum AuthFormatError {
    #[error("email  Format error:{0}")]
    EmailError(String),
    #[error("username Format error:{0}")]
    UsernameError(String),
}
