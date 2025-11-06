use std::{clone, sync::Arc};

use crate::auth_servicce::auth_service::AuthService;
use async_trait::async_trait;
use model::{
    auth_model::{PasswordHash, PasswordPlain, UserLogin, UserSingup},
    jwt::{AuthConfig, Claims, SessionRecordBuild},
    jwt_key_model::jwt::{SessionRecord, TokenResponse},
    users::{AccountStatus, Role, Users},
    users_model::users,
};
use tokio::sync::RwLock;
use use_case::{
    AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, HasherError, JwtRepo, JwtRepoError,
    LogoutResult, PolicyRepo, PolicyRepoError, RefreshRepo, RefreshToken, TimeSystemRepo, UserRepo,
    UserRepoError, VerifyStatus,
};
use uuid::Uuid;

// =============== fakes =============== //

struct FakeUserRepo;

#[async_trait]
impl UserRepo for FakeUserRepo {
    async fn get_password_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PasswordHash>, UserRepoError> {
        if username == "donut" {
            let pwh = PasswordHash::from_phc("argon2id$v=19$m=4096,t=3,p=1$SALT$HASH".to_string())
                .map_err(|e| UserRepoError::EnginError(e.to_string()))?;
            Ok(Some(pwh))
        } else if username == "master" {
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
        } else if username == "master" {
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
        Ok(())
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
        let phc_hash = PasswordHash::from_phc("argon2:fake_hash".to_string())
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
        } else if session_id == "master_ok_jti" {
            let fake_rt_hash = "master_ok_rt_hash".to_string();
            let cfg = AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            };
            let sessionrecord_master_ok = SessionRecordBuild::new(
                Uuid::new_v4(),
                Role::Master,
                AccountStatus::Active,
                &fake_rt_hash,
                1_700_000_999,
                1_700_000_000,
                1,
            )
            .cfg(cfg)
            .build();
            return Ok(Some(sessionrecord_master_ok));
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
    ) -> Result<use_case::RefreshToken, use_case::RefreshRepoError> {
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
        } else if token == "AT_FAKE_MASTER.JWT.TOKEN" {
            let master_ok_claims = Claims::new(
                "fake.sub".to_string(),
                "master_ok_jti".to_string(),
                900 as i64,
                700_000_000 as i64,
                1,
            );
            Ok(master_ok_claims)
        } else {
            Err(JwtRepoError::EnginFail("test".to_string()))
        }
    }
}
#[derive(Clone)]
struct FakePolicyMasterRepo {
    pub fake_policy_ver: Arc<RwLock<u32>>,
}
impl FakePolicyMasterRepo {
    pub fn new(fack_ver: u32) -> Self {
        Self {
            fake_policy_ver: Arc::new(RwLock::new(fack_ver)),
        }
    }
}
#[async_trait]
impl PolicyRepo for FakePolicyMasterRepo {
    async fn get_policy_version(&self) -> Result<u32, PolicyRepoError> {
        let res = self.fake_policy_ver.read().await;
        Ok(*res)
    }
    async fn bump_policy_version(&self) -> Result<u32, use_case::PolicyRepoError> {
        let mut ver = self.fake_policy_ver.write().await;
        *ver += 1;
        Ok(*ver)
    }
}

// =============== tests =============== //
#[cfg(test)]
mod tests {
    use crate::MasterService;

    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use use_case::MasterUseCase;

    // ----- test support -----
    fn make_master_service(fack_poliy: Arc<FakePolicyMasterRepo>) -> MasterService {
        MasterService::new(
            Arc::new(FakeUserRepo),
            fack_poliy,
            Arc::new(FakeJwtRepo),
            Arc::new(FakeAuthRepo),
            Arc::new(FakeHashRepo),
            Arc::new(FakeTimeRepo),
        )
    }
    fn make_auth_service(fack_poliy: Arc<FakePolicyMasterRepo>) -> AuthService {
        AuthService::new(
            Arc::new(FakeAuthRepo),
            Arc::new(FakeHashRepo),
            Arc::new(FakeUserRepo),
            Arc::new(FakeJwtRepo),
            Arc::new(FakeTimeRepo),
            Arc::new(FakeRefreshRepo),
            fack_poliy,
            AuthConfig {
                access_ttl: 900,
                refresh_ttl: 30 * 24 * 60 * 60,
                sesion_ttl: 30 * 24 * 60 * 60,
            },
        )
    }

    fn make_login_user_ok() -> UserLogin {
        UserLogin {
            username: "donut".to_string(),
            password_plain: PasswordPlain::form_vec(b"123456".to_vec()),
        }
    }
    fn make_login_master_ok() -> UserLogin {
        UserLogin {
            username: "master".to_string(),
            password_plain: PasswordPlain::form_vec(b"123456".to_vec()),
        }
    }
    fn make_token_user_ok() -> TokenResponse {
        let fake_rt = "RT_FAKE_TOKEN".to_string();
        TokenResponse::new("AT_FAKE.JWT.TOKEN".to_string(), &fake_rt, 999)
    }
    fn make_token_master_ok() -> TokenResponse {
        let fake_rt = "RT_FAKE_MASTER_TOKEN".to_string();
        TokenResponse::new("AT_FAKE_MASTER.JWT.TOKEN".to_string(), &fake_rt, 999)
    }
    pub struct OrderMaster {
        token: TokenResponse,
        user_id: Uuid,
        role: Role,
        status: AccountStatus,
    }
    fn make_order_master_ok(token: TokenResponse) -> OrderMaster {
        OrderMaster {
            token,
            user_id: Uuid::new_v4(),
            role: Role::User,
            status: AccountStatus::Active,
        }
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
    async fn master_update_user_status_ok() {
        let policy = FakePolicyMasterRepo::new(1);
        let auth = make_auth_service(Arc::new(policy.clone()));
        let master = make_master_service(Arc::new(policy.clone()));
        let order_user_login = make_token_user_ok();
        let order_master = make_token_master_ok();

        let opt_res_user = auth.refresh_token(order_user_login).await;
        assert!(
            opt_res_user.is_ok(),
            "expires Ok(...),got:{:?}",
            opt_res_user
        );
        let order_master = make_order_master_ok(order_master);
        let opt_res_master = master
            .update_user_status(
                order_master.token,
                order_master.user_id,
                order_master.role,
                order_master.status,
            )
            .await;
        println!("{opt_res_master:?}");
        assert!(
            opt_res_master.is_ok(),
            "expires Ok(...),got:{:?}",
            opt_res_master
        );
        let opt_res_user_refrech_after_master = auth.refresh_token(opt_res_user.unwrap()).await;
        assert!(opt_res_user_refrech_after_master.is_err());
        let err = opt_res_user_refrech_after_master.err().unwrap();
        assert_eq!(err, AuthUserCaseError::PolicyVersionMismatch);
    }
}
