use async_trait::async_trait;
use model::{
    auth_model::{PasswordHash, PasswordPlain, UserLogin, UserSingup},
    jwt::{AuthConfig, Claims, SessionRecordBuild},
    jwt_key_model::jwt::{SessionRecord, TokenResponse},
    users::{AccountStatus, Role, Users},
    users_model::users,
};
use use_case::{
    AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, HasherError, JwtRepo, JwtRepoError,
    LogoutResult, PolicyRepo, RefreshRepo, RefreshToken, TimeSystemRepo, UserRepo, UserRepoError,
    VerifyStatus,
};
use uuid::Uuid;

use crate::auth_servicce::auth_service::AuthService;

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
            let pwh = PasswordHash::from_phc("$argon2id$v=19$m=4096,t=3,p=1$SALT$HASH".to_string())
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
            status: AccountStatus::Active,
        };
        Ok(users)
    }

    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<model::users::Users>, use_case::UserRepoError> {
        unimplemented!()
    }
    async fn creat_user(
        &self,
        username: &str,
        email: &str,
        passwordhash: PasswordHash,
    ) -> Result<(), UserRepoError> {
        Ok(())
    }
    async fn check_username(&self, username: &str) -> Result<Option<()>, UserRepoError> {
        // id test is "donut"
        if username == "donut" {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    async fn list_user(&self) -> Result<Vec<model::users::Users>, use_case::UserRepoError> {
        unimplemented!()
    }
    async fn check_email(&self, email: &str) -> Result<Option<()>, UserRepoError> {
        // email_exists is "donut756@exists.com"
        if email == "donut756@exists.com" {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
    async fn update_user_status_role(
        &self,
        user_id: Uuid,
        user_status: AccountStatus,
        user_role: Role,
    ) -> Result<(), UserRepoError> {
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
        let pwd = String::from_utf8(cadidaie.0.to_vec()).unwrap();
        if pwd == "123456" {
            Ok(VerifyStatus::Pass)
        } else {
            Ok(VerifyStatus::Fail)
        }
    }
    async fn hash_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &str,
    ) -> Result<String, use_case::HasherError> {
        Ok(format!("hmac{rt_plain_base64}"))
    }
    async fn hashing_password_argon2(
        &self,
        ps_plain: model::auth_model::PasswordPlain,
    ) -> Result<PasswordHash, use_case::HasherError> {
        let phc_hash = PasswordHash::from_phc("$argon2:fake_hash".to_string())
            .map_err(|e| HasherError::FormatError(e.to_string()))?;
        Ok(phc_hash)
    }
    async fn varify_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &str,
        rt_hash_bash64: &str,
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
        session_id: &str,
    ) -> Result<Option<SessionRecord>, use_case::AuthRepoError> {
        if session_id == "fake_jti" {
            let fake_rt_hash = "fake_rt_hash".to_string();
            let cfg = AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            };
            let sessionrecord_ok = SessionRecordBuild::new(
                Uuid::new_v4(),
                Role::User,
                AccountStatus::Active,
                &fake_rt_hash,
                1_700_000_999,
                1_700_000_000,
                1,
            )
            .cfg(cfg)
            .build();
            return Ok(Some(sessionrecord_ok));
        } else if session_id == "old_policy_jti" {
            let fake_rt_hash = "fake_rt_hash".to_string();
            let cfg = AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            };
            let sessionrecord_old_policy = SessionRecordBuild::new(
                Uuid::new_v4(),
                Role::User,
                AccountStatus::Active,
                &fake_rt_hash,
                1_700_000_999,
                1_700_000_000,
                0,
            )
            .cfg(cfg)
            .build();
            return Ok(Some(sessionrecord_old_policy));
        } else if session_id == "refresh_expired_jti" {
            let fake_rt_hash = "fake_rt_hash".to_string();
            let cfg = AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            };
            let sessionrecord_old_policy = SessionRecordBuild::new(
                Uuid::new_v4(),
                Role::User,
                AccountStatus::Active,
                &fake_rt_hash,
                1_600_000_999,
                1_700_000_000,
                0,
            )
            .cfg(cfg)
            .build();
            return Ok(Some(sessionrecord_old_policy));
        } else {
            Ok(None)
        }
    }

    async fn kill_sesion_id(&self, session_id: &String) -> Result<bool, use_case::AuthRepoError> {
        if session_id == "fake_jti" {
            Ok(true)
        } else if session_id == "old_policy_jti" {
            Ok(true)
        } else {
            Ok(false)
        }
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
        rt_ttl: u32,
    ) -> Result<RefreshToken, use_case::RefreshRepoError> {
        let token_fake = "RT_FAKE_TOKEN".to_string();
        let fake_exp = now + rt_ttl as i64;
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
        at_ttl: u32,
        now: i64,
    ) -> Result<(String, i64), JwtRepoError> {
        Ok(("AT_FAKE.JWT.TOKEN".to_string(), { now + at_ttl as i64 }))
    }
    async fn decoder(&self, token: &str) -> Result<Claims, use_case::JwtRepoError> {
        //test token is "AT_FAKE.JWT.TOKEN"
        //jti is  fake_jti
        //now is fack
        if token == "AT_FAKE.JWT.TOKEN" {
            let fake_claims = Claims::new(
                "fake.sub".to_string(),
                "fake_jti".to_string(),
                900 as i64,
                700_000_000 as i64,
                1,
            );
            Ok(fake_claims)
        } else if token == "AT_NOTFOND.JWT.TOKEN" {
            let notfond_claims = Claims::new(
                "fake.sub".to_string(),
                "notfond_jti".to_string(),
                900 as i64,
                700_000_000 as i64,
                1,
            );
            Ok(notfond_claims)
        } else if token == "AT_OLD_POLICY.JWT.TOKEN" {
            let old_policy_claims = Claims::new(
                "fake.sub".to_string(),
                "old_policy_jti".to_string(),
                900 as i64,
                700_000_000 as i64,
                0,
            );
            Ok(old_policy_claims)
        } else if token == "AT_REFRESH_EXPIRED.JWT.TOKEN" {
            let refresh_expired_claims = Claims::new(
                "fake.sub".to_string(),
                "refresh_expired_jti".to_string(),
                900 as i64,
                700_000_000 as i64,
                1,
            );
            Ok(refresh_expired_claims)
        } else {
            Err(JwtRepoError::EnginFail("test".to_string()))
        }
    }
}
struct FakePolicyRepo;
#[async_trait]
impl PolicyRepo for FakePolicyRepo {
    async fn get_policy_version(&self) -> Result<u32, use_case::PolicyRepoError> {
        Ok(1)
    }
    async fn bump_policy_version(&self) -> Result<u32, use_case::PolicyRepoError> {
        unimplemented!()
    }
}

// =============== tests =============== //
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};

    // ----- test support -----
    fn make_auth_service() -> AuthService {
        AuthService::new(
            Arc::new(FakeAuthRepo),
            Arc::new(FakeHashRepo),
            Arc::new(FakeUserRepo),
            Arc::new(FakeJwtRepo),
            Arc::new(FakeTimeRepo),
            Arc::new(FakeRefreshRepo),
            Arc::new(FakePolicyRepo),
            AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            },
        )
    }

    fn make_login_ok() -> UserLogin {
        UserLogin {
            username: "donut".to_string(),
            password_plain: PasswordPlain::form_vec(b"123456".to_vec()),
        }
    }

    fn make_login_wrong_password() -> UserLogin {
        UserLogin {
            username: "donut".to_string(),
            password_plain: PasswordPlain::form_vec(b"WRONG".to_vec()),
        }
    }
    fn make_login_wrong_username() -> UserLogin {
        UserLogin {
            username: "someone".to_string(),
            password_plain: PasswordPlain::form_vec(b"1234".to_vec()),
        }
    }
    fn make_singup_ok() -> UserSingup {
        UserSingup {
            username: "donut_dont_exists".to_string(),
            email: "donut@donut_dont_exists".to_string(),
            password_plain: PasswordPlain::form_vec(b"1234".to_vec()),
        }
    }
    fn make_singup_username_exists() -> UserSingup {
        UserSingup {
            username: "donut".to_string(),
            email: "donut@donut_dont_exists".to_string(),
            password_plain: PasswordPlain::form_vec(b"1234".to_vec()),
        }
    }
    fn make_singup_email_exissts() -> UserSingup {
        UserSingup {
            username: "donut".to_string(),
            email: "donut@donutexists".to_string(),
            password_plain: PasswordPlain::form_vec(b"1234".to_vec()),
        }
    }
    fn make_token_ok() -> TokenResponse {
        let fake_rt = "RT_FAKE_TOKEN".to_string();
        TokenResponse::new("AT_FAKE.JWT.TOKEN".to_string(), &fake_rt, 999)
    }
    fn make_token_notfond() -> TokenResponse {
        let fake_rt = "RT_NOTFOND_TOKEN".to_string();
        TokenResponse::new("AT_NOTFOND.JWT.TOKEN".to_string(), &fake_rt, 999)
    }
    fn make_token_old_policy() -> TokenResponse {
        let fake_rt = "RT_OLD_POLICY_TOKEN".to_string();
        TokenResponse::new("AT_OLD_POLICY.JWT.TOKEN".to_string(), &fake_rt, 999)
    }
    fn make_token_refresh_expired() -> TokenResponse {
        let fake_rt = "RT_REFRESH_EXPIRED_TOKEN".to_string();
        TokenResponse::new("AT_REFRESH_EXPIRED.JWT.TOKEN".to_string(), &fake_rt, 999)
    }

    // ----- tests -----

    #[tokio::test]
    async fn login_success() {
        let svc = make_auth_service();
        let order = make_login_ok();
        let res = svc.login(order).await;
        println!("{res:?}");
        assert!(res.is_ok());
        let token = res.unwrap();
        assert_eq!(token.access_token, "AT_FAKE.JWT.TOKEN");
        assert_eq!(token.refresh_token, "RT_FAKE_TOKEN");
        assert_eq!(token.expires_in, 1700000900);
    }

    #[tokio::test]
    async fn login_wrong_password() {
        let svc = make_auth_service();
        let order = make_login_wrong_password();
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
        let svc = make_auth_service();
        let order = make_login_wrong_username();
        let res = svc.login(order).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        match err {
            AuthUserCaseError::Authentication => {}
            _ => panic!("expected Authentication, got {err:?}"),
        }
    }

    #[tokio::test]
    async fn singup_success() {
        let svc = make_auth_service();
        let order = make_singup_ok();
        let res = svc.singup(order).await;
        println!("{res:?}");
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn singup_username_exists() {
        let svc = make_auth_service();
        let order = make_singup_username_exists();
        let res = svc.singup(order).await;
        println!("{res:?}");
        assert!(res.is_err());
        let err: AuthUserCaseError = res.err().unwrap();
        match err {
            AuthUserCaseError::BadRequet => {}
            _ => panic!("expected AuthUserCaseError, got {err:?}"),
        }
    }
    #[tokio::test]
    async fn singup_email_exists() {
        let svc = make_auth_service();
        let order = make_singup_email_exissts();
        let res = svc.singup(order).await;
        println!("{res:?}");
        assert!(res.is_err());
        let err = res.err().unwrap();
        match err {
            AuthUserCaseError::BadRequet => {}
            _ => panic!("expected AuthUserCaseError, got {err:?}"),
        }
    }
    #[tokio::test]
    async fn logout_success() {
        let svc = make_auth_service();
        let order = make_token_ok();
        let res = svc.logout(order.access_token).await;
        assert!(res.is_ok(), "expected Ok(...), got: {:?}", res);
        let out = res.unwrap();
        assert_eq!(out, LogoutResult::SessionTerminated);
    }
    #[tokio::test]
    async fn logout_notfond() {
        let svc = make_auth_service();
        let order = make_token_notfond();
        let res = svc.logout(order.access_token).await;
        assert!(res.is_ok(), "expected Ok(...), got: {:?}", res);
        let out = res.unwrap();
        assert_eq!(out, LogoutResult::SessionNotFond);
    }
    #[tokio::test]
    async fn refresh_token_ok() {
        let svc = make_auth_service();
        let order = make_token_ok();
        let res = svc.refresh_token(order).await;
        assert!(res.is_ok(), "expected Ok(...), got: {:?}", res);
    }
    #[tokio::test]
    async fn refresh_token_old_policy() {
        let svc = make_auth_service();
        let order = make_token_old_policy();
        let res = svc.refresh_token(order).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err, AuthUserCaseError::PolicyVersionMismatch);
    }
    #[tokio::test]
    async fn refresh_token_refresh_not_fond() {
        let svc = make_auth_service();
        let order = make_token_notfond();
        let res = svc.refresh_token(order).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err, AuthUserCaseError::SessionNotFond);
    }
    #[tokio::test]
    async fn refresh_token_refresh_expired() {
        let svc = make_auth_service();
        let order = make_token_refresh_expired();
        let res = svc.refresh_token(order).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err, AuthUserCaseError::RefreshExpired);
    }
}
