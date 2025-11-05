//for query data and work flow case //
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: AcconutStatus,
}
//#[derive(sqlx::Type, Debug)]
//#[sqlx(type_name = "acconut_status", rename_all = "lowercase")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcconutStatus {
    Pending,
    Active,
    Susspended,
    Disable,
}

//#[derive(sqlx::Type, Debug)]
//#[sqlx(type_name = "user_role", rename_all = "lowercase")]

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Role {
    User,
    Admin,
    Master,
}
