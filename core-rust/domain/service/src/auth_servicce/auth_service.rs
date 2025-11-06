use async_trait::async_trait;
use model::{
    auth_model::{UserLogin, UserSingup},
    jwt::SessionRecordBuild,
    jwt_key_model::jwt::{AuthConfig, TokenResponse},
};
use std::sync::Arc;
use tracing::{error, info, warn};
use use_case::{
    AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, JwtRepo, LogoutResult, RefreshRepo,
    TimeSystemRepo, UserRepo, VerifyStatus,PolicyRepo
};

pub struct AuthService {
    pub auth_repo: Arc<dyn AuthRepo + Send + Sync>,
    pub hash_repo: Arc<dyn HashRepo + Send + Sync>,
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
    pub time_repo: Arc<dyn TimeSystemRepo + Send + Sync>,
    pub rt_repo: Arc<dyn RefreshRepo + Send + Sync>,
    pub policy_repo: Arc<dyn PolicyRepo +Send +Sync>,
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
        policy_repo: Arc<dyn PolicyRepo + Send +Sync>,
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
            policy_repo,
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
                    .gen_refresh_token_base64(now, self.auth_cfg.refresh_ttl )
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
                let policy_ver=self.policy_repo.get_policy_version().await.map_err(|e|{
                    error!(error=%e,"policyrepo fail while get policy version");
                    AuthUserCaseError::PolicyRepoError(e.to_string())
                })?;
                let session_record = SessionRecordBuild::new(
                    user_row.id,
                    user_row.role,
                    user_row.status,
                    &rt_hash,
                    rt_token.token_exp,
                    now,
                    policy_ver,
                )
                .cfg(self.auth_cfg.clone())
                .build();
                self.auth_repo
                    .create_session(&session_record)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"create_session fail");
                        AuthUserCaseError::AuthRepoFail(e.to_string())
                    })?;
                let (at_token, at_exp) = self
                    .jwt_repo
                    .encoder(&session_record, self.auth_cfg.access_ttl ,now)
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
        //parallel check
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
    async fn logout(&self, order: TokenResponse) -> Result<LogoutResult, AuthUserCaseError> {
        // fn decoder have validate inside //
        let claim = self
            .jwt_repo
            .decoder(&order.access_token)
            .await
            .map_err(|e| {
                error!(error=%e,ac_token=%order.access_token,"jwtRepofail ");
                AuthUserCaseError::JwtRepofail(e.to_string())
            })?;
        let deleted = self
            .auth_repo
            .kill_sesion_id(&claim.jti)
            .await
            .map_err(|e| {
                error!(error=%e,sses_id=%claim.jti,"auth_repo fail ");
                AuthUserCaseError::AuthRepoFail(e.to_string())
            })?;
        if deleted {
            info!(sses_id=%claim.jti,"SessionTerminated");
            Ok(LogoutResult::SessionTerminated)
        } else {
            info!(sses_id=%claim.jti,"SessionNotFond");
            Ok(LogoutResult::SessionNotFond)
        }
    }
    async fn refresh_token(
        &self,
        order: TokenResponse,
    ) -> Result<TokenResponse, AuthUserCaseError> {
        // decoder claims
        let order_claims = self
            .jwt_repo
            .decoder(&order.access_token)
            .await
            .map_err(|e| {
                error!(error=%e,ac_token=%order.access_token,"jwtRepofail");
                AuthUserCaseError::JwtRepofail(e.to_string())
            })?;
        // get opt sess_id
        let opt_session_record = self
            .auth_repo
            .get_sesion_by_sess_id(&order_claims.jti)
            .await
            .map_err(|e| {
                error!(error=%e,sess_id=%order_claims.jti,"auth_repo fail");
                AuthUserCaseError::AuthRepoFail(e.to_string())
            })?;
        // check opt sess_id
        let session_record = match opt_session_record {
            Some(s) => s,
            None => {
                warn!(sess_id=%order_claims.jti, "session not found (maybe TTL) -> force re-login");
                return Err(AuthUserCaseError::SessionNotFond);
            }
        };
        //hash check rt_order and rt of system
        let verify_rt_status = self
            .hash_repo
            .varify_rt_hmac_sha256_base64(&order.refresh_token, &session_record.rt_hash)
            .await
            .map_err(|e| {
                error!(error=%e,rt_plain=%order.refresh_token,"hash_repo fail");
                AuthUserCaseError::HashingFail(e.to_string())
            })?;
        // check rt exp
        let now = self.time_repo.now().await;
        if now > session_record.rt_exp {
            warn!(sess_id=%session_record.session_id, "refresh token expired -> kill session");
            let _ = self
                .auth_repo
                .kill_sesion_id(&session_record.session_id)
                .await
                .map_err(|e| {
                    error!(error=%e, "auth_repo kill fail (rt expired)");
                    AuthUserCaseError::AuthRepoFail(e.to_string())
                })?;
            return Err(AuthUserCaseError::RefreshExpired);
        }
        match verify_rt_status {
            VerifyStatus::Pass => {
                //check policy_ver when admin or master work
                    let policy_ver=self.policy_repo.get_policy_version().await.map_err(|e|{
                    error!(error=%e,"policyrepo fail while get policy version");
                    AuthUserCaseError::PolicyRepoError(e.to_string())
                })?;

                if session_record.policy_ver != policy_ver{
                    warn!(user_id=%session_record.user_id, 
                            old=%session_record.policy_ver, 
                            now=%policy_ver, 
                            "policy changed, Kill session , force re-login");
                    let _ = self
                        .auth_repo
                        .kill_sesion_id(&session_record.session_id)
                        .await
                        .map_err(|e| {
                            error!(error=%e,"auth_repo fail while on policy mismatch");
                            AuthUserCaseError::AuthRepoFail(e.to_string())
                        })?;
                    //TODO case cant kill session
                    return Err(AuthUserCaseError::PolicyVersionMismatch);
                }
                // gen new at 
                let now =self.time_repo.now().await;
                let (new_at, new_at_exp) = self
                    .jwt_repo
                    .encoder(&session_record, self.auth_cfg.access_ttl ,now)
                    .await
                    .map_err(|e| {
                        error!(error=%e,"encoder fail");
                        AuthUserCaseError::JwtRepofail(e.to_string())
                    })?;
                // and now rt is work respon old
                let new_token = TokenResponse::new(
                        new_at, 
                        &order.refresh_token, 
                        new_at_exp as u32);
                Ok(new_token)
            }
            VerifyStatus::Fail => {
                warn!(rt_hash=%session_record.rt_hash,rt_plain=%order.refresh_token,"bad BadRequet");
                Err(AuthUserCaseError::BadRequet)
            }
            VerifyStatus::Corrupted => {
                //TODO hendel this case
                todo!()
            }
        }
    }
}
