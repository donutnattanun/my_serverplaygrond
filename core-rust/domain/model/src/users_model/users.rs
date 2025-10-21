//for query data and work flow case //
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Users {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub status: Option<AcconutStatus>,
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
