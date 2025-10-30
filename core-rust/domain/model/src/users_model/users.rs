//for query data and work flow case //
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: AcconutStatus,
}
//#[derive(sqlx::Type, Debug)]
//#[sqlx(type_name = "acconut_status", rename_all = "lowercase")]
#[derive(Debug, Clone)]
pub enum AcconutStatus {
    Pending,
    Active,
    Susspended,
    Disable,
}

//#[derive(sqlx::Type, Debug)]
//#[sqlx(type_name = "user_role", rename_all = "lowercase")]

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Admin,
    Master,
}
