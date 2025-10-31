//tests/auth_login_tests.rs

use std::sync::Arc;

use async_trait::async_trait;
use model::{
    auth_model::{PasswordHash, UserLogin},
    jwt::AuthConfig,
    jwt_key_model::jwt::{SessionRecord, TokenResponse},
    users::{AcconutStatus, Role, Users},
    users_model::users,
};
use use_case::{
    AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, JwtRepo, RefreshRepo, RefreshToken,
    TimeSystemRepo, UserRepo, UserRepoError, VerifyStatus,
};
use uuid::Uuid;

use crate::auth_service::AuthService;

// =============== fakes =============== //

struct FakeUserRepo;

#[async_trait]
impl UserRepo for FakeUserRepo {
    // สมมุติว่าของจริงคืน Option<String> = phc
    async fn get_password_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PasswordHash>, UserRepoError> {
        if username == "donut" {
            // สมมุติว่าใน DB เก็บ phc แบบนี้
            let pwh = PasswordHash::from_phc("argon2id$v=19$m=4096,t=3,p=1$SALT$HASH".to_string())
                .map_err(|e| UserRepoError::EnginError(e.to_string()))?;
            Ok(Some(pwh))
        } else {
            Ok(None)
        }
    }
    async fn get_user_by_username(&self, username: &str) -> Result<users::Users, UserRepoError> {
        let users = Users {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: format!("{username}@example.com"),
            role: Role::User,
            status: AcconutStatus::Active,
        };
        Ok(users)
    }

    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<model::users::Users>, use_case::UserRepoError> {
        unimplemented!()
    }
    async fn creat_user(&self, user: model::users::Users) -> Result<(), use_case::UserRepoError> {
        unimplemented!()
    }
    async fn list_user(&self) -> Result<Vec<model::users::Users>, use_case::UserRepoError> {
        unimplemented!()
    }
}

struct FakeHashRepo;

#[async_trait]
impl HashRepo for FakeHashRepo {
    async fn varify_password_argon2(
        &self,
        phc: model::auth_model::PasswordHash,
        cadidaie: model::auth_model::PasswordPlain,
    ) -> Result<VerifyStatus, use_case::HasherError> {
        // เทสต์ง่าย ๆ: ถ้า user ส่ง password = "123456" ให้ผ่าน
        let pwd = String::from_utf8(cadidaie.0.to_vec()).unwrap();
        if pwd == "123456" {
            Ok(VerifyStatus::Pass)
        } else {
            Ok(VerifyStatus::Fail)
        }
    }
    async fn hash_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &String,
    ) -> Result<String, use_case::HasherError> {
        Ok(format!("hmac{rt_plain_base64}"))
    }
    async fn hashing_password_argon2(
        &self,
        ps_plain: model::auth_model::PasswordPlain,
    ) -> Result<PasswordHash, use_case::HasherError> {
        unimplemented!()
    }
    async fn varify_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &String,
        rt_hash_bash64: &String,
    ) -> Result<VerifyStatus, use_case::HasherError> {
        Ok(VerifyStatus::Pass)
    }
}

struct FakeAuthRepo;

#[async_trait]
impl AuthRepo for FakeAuthRepo {
    async fn create_session(&self, session: &SessionRecord) -> Result<(), use_case::AuthRepoError> {
        Ok(())
    }
    async fn get_sesion_by_sess_id(
        &self,
        session_id: &String,
    ) -> Result<SessionRecord, use_case::AuthRepoError> {
        unimplemented!()
    }
    async fn kill_sesion_id(&self, session_id: &String) -> Result<(), use_case::AuthRepoError> {
        unimplemented!()
    }
}

struct FakeTimeRepo;

#[async_trait]
impl TimeSystemRepo for FakeTimeRepo {
    async fn now(&self) -> i64 {
        1_700_000_000 // fixed time
    }
}

struct FakeRefreshRepo;

#[async_trait]
impl RefreshRepo for FakeRefreshRepo {
    async fn gen_refresh_token_base64(
        &self,
        now: i64,
        rt_ttl: i64,
    ) -> Result<use_case::RefreshToken, use_case::RefreshRepoError> {
        let token_fake = "RT_FAKE_TOKEN".to_string();
        let fake_exp = now + rt_ttl;

        let token = RefreshToken::new(token_fake, fake_exp);
        Ok(token)
    }
}

struct FakeJwtRepo;

#[async_trait]
impl JwtRepo for FakeJwtRepo {
    async fn encoder(
        &self,
        session: &SessionRecord,
        at_ttl: i64,
    ) -> Result<(String, i64), use_case::JwtRepoError> {
        Ok(("AT_FAKE.JWT.TOKEN".to_string(), at_ttl))
    }
    async fn decoder(&self, token: String) -> Result<SessionRecord, use_case::JwtRepoError> {
        unimplemented!()
    }
}

// =============== tests =============== //

#[tokio::test]
async fn login_success() {
    let svc = AuthService::new(
        Arc::new(FakeAuthRepo),
        Arc::new(FakeHashRepo),
        Arc::new(FakeUserRepo),
        Arc::new(FakeJwtRepo),
        Arc::new(FakeTimeRepo),
        Arc::new(FakeRefreshRepo),
        AuthConfig {
            access_ttl: 900,
            refresh_ttl: 30 * 24 * 60 * 60,
            sesion_ttl: 30 * 24 * 60 * 60,
        },
    );

    let order = UserLogin {
        username: "donut".to_string(),
        // struct นายอาจเป็น PasswordPlain(Zeroizing<Vec<u8>>)
        password_plain: model::auth_model::PasswordPlain::form_vec(b"123456".to_vec()),
    };

    let res = svc.login(order).await;
    println!("{res:?}");
    assert!(res.is_ok());
    let token = res.unwrap();

    assert_eq!(token.access_token, "AT_FAKE.JWT.TOKEN");
    assert_eq!(token.refresh_token, "RT_FAKE_TOKEN");
    assert_eq!(token.expires_in, 900);
}

#[tokio::test]
async fn login_wrong_password() {
    let svc = AuthService::new(
        Arc::new(FakeAuthRepo),
        Arc::new(FakeHashRepo),
        Arc::new(FakeUserRepo),
        Arc::new(FakeJwtRepo),
        Arc::new(FakeTimeRepo),
        Arc::new(FakeRefreshRepo),
        AuthConfig {
            access_ttl: 900,
            refresh_ttl: 30 * 24 * 60 * 60,
            sesion_ttl: 30 * 24 * 60 * 60,
        },
    );

    let order = UserLogin {
        username: "donut".to_string(),
        password_plain: model::auth_model::PasswordPlain::form_vec(b"WRONG".to_vec()),
    };

    let res = svc.login(order).await;
    assert!(res.is_err());
    let err = res.err().unwrap();
    match err {
        AuthUserCaseError::BadRequet => {}
        _ => panic!("expected BadRequet, got {err:?}"),
    }
}

#[tokio::test]
async fn login_user_not_found() {
    let svc = AuthService::new(
        Arc::new(FakeAuthRepo),
        Arc::new(FakeHashRepo),
        Arc::new(FakeUserRepo),
        Arc::new(FakeJwtRepo),
        Arc::new(FakeTimeRepo),
        Arc::new(FakeRefreshRepo),
        AuthConfig {
            access_ttl: 900,
            refresh_ttl: 30 * 24 * 60 * 60,
            sesion_ttl: 30 * 24 * 60 * 60,
        },
    );

    let order = UserLogin {
        username: "unknown-user".to_string(),
        password_plain: model::auth_model::PasswordPlain::form_vec(b"123456".to_vec()),
    };

    let res = svc.login(order).await;
    assert!(res.is_err());
    let err = res.err().unwrap();
    match err {
        AuthUserCaseError::Authentication => {}
        _ => panic!("expected Authentication, got {err:?}"),
    }
}
