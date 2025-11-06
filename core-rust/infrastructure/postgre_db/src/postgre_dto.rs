use model::{
    auth_model::PasswordHash,
    users::{AccountStatus, Role},
};
use use_case::UserRepoError;
use uuid::Uuid;

//----dto-----//
#[derive(sqlx::FromRow)]
pub struct PasswordHashDto {
    pub password_hash: String,
}
impl TryFrom<PasswordHashDto> for PasswordHash {
    type Error = UserRepoError;

    fn try_from(value: PasswordHashDto) -> Result<Self, Self::Error> {
        let v = PasswordHash::from_phc(value.password_hash)
            .map_err(|e| UserRepoError::FormatError(e.to_string()))?;
        Ok(v)
    }
}

#[derive(sqlx::FromRow)]
pub struct UsersRowDto {
    // id auto den by db
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub status: AccountStatus,
}
impl UsersRowDto {
    pub fn make_user_row_default(username: &str, email: &str, phc: PasswordHash) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            password_hash: phc.phc,
            role: Role::User,
            status: AccountStatus::Pending,
        }
    }
}
