use uuid::Uuid;

pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
