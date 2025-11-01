use async_trait::async_trait;
use model::{
    auth_model::{PasswordPlain, UserLogin, UserSingup},
    jwt::SessionRecordBuild,
    jwt_key_model::jwt::{AuthConfig, TokenResponse},
};
use std::sync::Arc;
use tracing::{error, info, warn};
use use_case::{
    AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, JwtRepo, RefreshRepo, TimeSystemRepo,
    UserRepo, VerifyStatus,
};

pub struct AuthService {
    pub auth_repo: Arc<dyn AuthRepo + Send + Sync>,
    pub hash_repo: Arc<dyn HashRepo + Send + Sync>,
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
    pub time_repo: Arc<dyn TimeSystemRepo + Send + Sync>,
    pub rt_repo: Arc<dyn RefreshRepo + Send + Sync>,
    pub auth_cfg: AuthConfig,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<dyn AuthRepo + Send + Sync>,
        hash_repo: Arc<dyn HashRepo + Send + Sync>,
        user_repo: Arc<dyn UserRepo + Send + Sync>,
        jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
        time_repo: Arc<dyn TimeSystemRepo + Send + Sync>,
        rt_repo: Arc<dyn RefreshRepo + Send + Sync>,
        auth_cfg: AuthConfig,
    ) -> Self {
        Self {
            auth_repo,
            hash_repo,
            user_repo,
            jwt_repo,
            auth_cfg,
            time_repo,
            rt_repo,
        }
    }
}
#[async_trait]
impl AuthUserCase for AuthService {
    async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError> {
        info!(username=%order.username,"Login attempt");
        let opt_phc_db = self
            .user_repo
            .get_password_by_username(&order.username)
            .await
            .map_err(|e| {
                warn!(user=%order.username,error=%e,"db fail error");
                AuthUserCaseError::DbFail(e.to_string())
            })?;
        let phc_db = match opt_phc_db {
            Some(phc) => phc,
            None => {
                error!(username=%order.username, "authentication");
                return Err(AuthUserCaseError::Authentication);
            }
        };
        let verify = self
            .hash_repo
            .varify_password_argon2(phc_db, order.password_plain)
            .await
            .map_err(|e| {
                error!(error=%e,"hash repo error");
                AuthUserCaseError::HashingFail(e.to_string())
            })?;
        match verify {
            VerifyStatus::Pass => {
                //gen token session --//
                let user_row = self
                    .user_repo
                    .get_user_by_username(&order.username)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"get_user_by_username fail");
                        AuthUserCaseError::DbFail(e.to_string())
                    })?;
                let now = self.time_repo.now().await;
                let rt_token = self
                    .rt_repo
                    .gen_refresh_token_base64(now, self.auth_cfg.refresh_ttl as i64)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"gen_refresh_token_base64 fail");
                        AuthUserCaseError::RefechFail(e.to_string())
                    })?;
                let rt_hash = self
                    .hash_repo
                    .hash_rt_hmac_sha256_base64(&rt_token.token_plain)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"hash fail ");
                        AuthUserCaseError::HashingFail(e.to_string())
                    })?;
                let session_record = SessionRecordBuild::new(
                    user_row.id,
                    user_row.role,
                    user_row.status,
                    &rt_hash,
                    rt_token.token_exp,
                    now,
                )
                .cfg(self.auth_cfg.clone())
                .build();
                self.auth_repo
                    .create_session(&session_record)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"create_session fail");
                        AuthUserCaseError::CashingFail(e.to_string())
                    })?;
                let (at_token, at_exp) = self
                    .jwt_repo
                    .encoder(&session_record, self.auth_cfg.access_ttl as i64)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"jwt encode fail ");
                        AuthUserCaseError::JwtRepofail(e.to_string())
                    })?;
                let token_respon =
                    TokenResponse::new(at_token, &rt_token.token_plain, at_exp as u32);
                return Ok(token_respon);
            }
            VerifyStatus::Fail => {
                warn!(usermane=%order.username,"bad passward");
                return Err(AuthUserCaseError::BadRequet);
            }
            VerifyStatus::Corrupted => {
                warn!(user=%order.username,"Corrupted password");
                return Err(AuthUserCaseError::Corrupted);
            }
        }
    }
    async fn singup(&self, order: UserSingup) -> Result<(), AuthUserCaseError> {
        info!(username=%order.username,"singup attempt");
        let (username_exists, email_exists) = tokio::try_join!(
            async {
                let exists = self
                    .user_repo
                    .check_username(&order.username)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"user repo ");
                        AuthUserCaseError::DbFail(e.to_string())
                    })?
                    .is_some();
                if exists {
                    warn!(order=%order.username,"BadRequet username singup :");
                }
                Ok::<bool, AuthUserCaseError>(exists)
            },
            async {
                let exists = self
                    .user_repo
                    .check_email(&order.email)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"user repo ");
                        AuthUserCaseError::DbFail(e.to_string())
                    })?
                    .is_some();
                if exists {
                    warn!(order=%order.username,"BadRequet Email singup :");
                }
                Ok::<bool, AuthUserCaseError>(exists)
            }
        )?;
        if username_exists || email_exists {
            return Err(AuthUserCaseError::BadRequet);
        }
        let phc_hash = self
            .hash_repo
            .hashing_password_argon2(order.password_plain)
            .await
            .map_err(|e| {
                error!(error=%e,"hash repo fail");
                AuthUserCaseError::HashingFail(e.to_string())
            })?;
        self.user_repo
            .creat_user(&order.username, &order.email, phc_hash)
            .await
            .map_err(|e| {
                error!(error=%e,"creat_user fail");
                AuthUserCaseError::DbFail(e.to_string())
            })?;
        Ok(())
    }
    async fn logout(&self) {}
    async fn refresh(&self, order: TokenResponse) -> Result<TokenResponse, AuthUserCaseError> {
        unimplemented!()
    }
}
