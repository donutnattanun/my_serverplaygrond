use uuid::Uuid;

//-----user_DTO------//
#[derive(sqlx::FromRow, Debug)]
pub struct UserRepoDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl From<UserRepoDto> for model::Users {
    fn from(value: UserRepoDto) -> Self {
        model::Users::from_db(value.id, value.username, value.email, value.password_hash)
    }
}
//--user auth --//
#[derive(sqlx::FromRow, Debug)]
pub struct UserAuthRepoDto {
    pub username: String,
    pub password_hash: String,
}
//-- --//
//-- dao for infar tarit --//
//should add and tryform for model entity for use in usecase and service later //

//-- --//
#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("db error {0}")]
    Db(String),
}
