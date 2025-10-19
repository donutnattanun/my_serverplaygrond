use serde::{Deserialize, Serialize};
use use_case::{AuthError, UserLoginOrder, UserSingupOrder, UserUseCaseDto, Valid};
use uuid::Uuid;
#[derive(Deserialize)]
pub struct UserSingupReq {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl TryFrom<UserSingupReq> for Valid<UserSingupOrder> {
    type Error = AuthError;
    fn try_from(value: UserSingupReq) -> Result<Self, Self::Error> {
        Valid::<UserSingupOrder>::new(value.username, value.email, value.password)
    }
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

#[derive(Deserialize)]
pub struct UserLoginReq {
    pub username: String,
    pub password: String,
}
impl TryFrom<UserLoginReq> for Valid<UserLoginOrder> {
    type Error = AuthError;
    fn try_from(value: UserLoginReq) -> Result<Self, Self::Error> {
        //add logic valid in did fn
        Valid::<UserLoginOrder>::new(value.username, value.password)
    }
}
//--validate_test--//
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid_login_order_ok() {
        let order = UserLoginReq {
            username: "donut".into(),
            password: "12345".into(),
        };
        let got: Valid<UserLoginOrder> = order.try_into().expect("valid loginOrder");
        assert_eq!(got.0.username, "donut");
        assert_eq!(got.0.password, "12345");
    }
    #[test]
    fn valid_login_order_fail_username() {
        let order = UserLoginReq {
            username: " ".into(),
            password: "12345".into(),
        };
        let got: Result<Valid<UserLoginOrder>, AuthError> = order.try_into();
        assert!(matches!(got, Err(AuthError::Invalid)));
    }
    #[test]
    fn valid_login_order_fail_passwaord() {
        let order = UserLoginReq {
            username: "donut".into(),
            password: "123".into(),
        };
        let got = Valid::try_from(order);
        assert!(matches!(got, Err(AuthError::Invalid)));
    }
}
