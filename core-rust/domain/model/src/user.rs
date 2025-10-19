use argon2::password_hash;
use thiserror::Error;
use uuid::Uuid;

pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
#[derive(Debug, Error)]
pub enum UsersModelError {
    #[error("Username is empty")]
    EmptyUsername,
    #[error("Invalid email : {0}")]
    IncalidEmail(String),
    #[error("Missing hash")]
    MissingHash,
}
impl Users {
    pub fn new(
        id: Uuid,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<Self, UsersModelError> {
        if username.is_empty() {
            return Err(UsersModelError::EmptyUsername);
        }
        if !email.contains("@") {
            return Err(UsersModelError::IncalidEmail(email));
        }
        if password_hash.is_empty() {
            return Err(UsersModelError::MissingHash);
        }
        Ok(Self {
            id,
            username,
            email,
            password_hash,
        })
    }
    pub fn from_db(id: Uuid, username: String, email: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
        }
    }
}
