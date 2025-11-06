//for query data and work flow case //
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: AccountStatus,
}
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    Pending,
    Active,
    Suspended,
    Disabled,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Role {
    User,
    Admin,
    Master,
}
