use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub password_hash: String,
}
