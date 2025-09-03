use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
