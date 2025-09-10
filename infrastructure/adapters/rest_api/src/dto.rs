use serde::{Deserialize, Serialize};
use use_case::UserUseCaseDto;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct UserReq {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct UserResp {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
impl From<UserUseCaseDto> for UserResp {
    fn from(u: UserUseCaseDto) -> Self {
        let id_str = Uuid::parse_str(&u.id).unwrap();
        Self {
            id: id_str,
            username: u.username,
            email: u.email,
        }
    }
}
