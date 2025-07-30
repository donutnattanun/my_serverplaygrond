use crate::db::models::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub username: String,
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
    fn from(user: User) -> Self {
        Self {
            id: user.user_id,
            user_name: user.user_name,
            email: user.user_email,
        }
    }
}
