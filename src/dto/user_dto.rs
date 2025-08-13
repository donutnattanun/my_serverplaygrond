use crate::db::user_models::{self, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub user_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
}

// when call this impl we can call
// User.into
// rust auto it to
// impl into<UserResponse> for User
// impl <T,U> into<U> for T
// when
//     U : From T
impl From<User> for UserResponse {
    fn from(user_models: User) -> Self {
        Self {
            id: user_models.user_id,
            user_name: user_models.user_name,
            email: user_models.user_email,
        }
    }
}

impl From<User> for UserRequest {
    fn from(user_models: User) -> Self {
        Self {
            user_name: user_models.user_name,
            email: user_models.user_email,
            password: user_models.password_hash,
        }
    }
}
