use model::{auth_model::UserLogin, jwt::{SessionRecord, SessionRecordBuild}, jwt_key_model::jwt::{TokenResponse,AuthConfig};
use std::sync::Arc;
use tracing::{error, info, warn};
use use_case::{AuthRepo, AuthUserCase, AuthUserCaseError, HashRepo, UserRepo, VerifyStatus,JwtRepo};

pub struct AuthService {
    pub auth_repo: Arc<dyn AuthRepo + Send + Sync>,
    pub hash_repo: Arc<dyn HashRepo + Send + Sync>,
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub jwt_repo: Arc<dyn JwtRepo + Send +Sync>, 
    pub auth_cfg: AuthConfig,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<dyn AuthRepo + Send + Sync>,
        hash_repo: Arc<dyn HashRepo + Send + Sync>,
        user_repo: Arc<dyn UserRepo + Send + Sync>,
        jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
        auth_cfg: AuthConfig,
    ) -> Self {
        Self {
            auth_repo,
            hash_repo,
            user_repo,
            jwt_repo,
            auth_cfg,
        }
    }
}
#[async_trait::async_trait]
impl AuthUserCase for AuthService {
    async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError> {
        tracing::instrument(skip(self, order.password_plain));
        info!(username=%order.username,"Login attempt");
        let opt_phc_db = self
            .user_repo
            .get_password_by_username(&order.username)
            .await
            .map_err(|e| {
                warn!(user=%order.username,error=%e,"db fail error");
                AuthUserCaseError::DbFail(e.to_string())
            })?;
        let phc_db = opt_phc_db.ok_or({
            error!(username=%order.username,
                    "authentication ");
            return Err((AuthUserCaseError::Authentication));
        })?;
        let verify = self
            .hash_repo
            .varify(phc_db, order.password_plain)
            .await
            .map_err(|e| {
                error!(error=%e,"hash error error");
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
                        error!(error=%e,"db engin");
                        AuthUserCaseError::DbFail(e.to_string())
                    })?;
                let rt_hash=;
               let sesion_rc=SessionRecordBuild::new(
                    user_row.id,
                    user_row.role,
                    user_row.status,
                    rt_hash)
                    .cfg(self.auth_cfg);
            },
            VerifyStatus::Fail => {
                warn!(usermane=%order.username,"bad passward");
                return Err((AuthUserCaseError::BadRequet));
            },
            VerifyStatus::Corrupted => {
                warn!(user=%order.username,"Corrupted password");
                return Err((AuthUserCaseError::Corrupted));
            }
        };

    }
    async fn singup(&self, order: model::auth_model::UserSingup) -> Result<(), AuthUserCaseError> {}
    async fn logout(&self) {}
    async fn refresh(&self, order: TokenResponse) -> Result<TokenResponse, AuthUserCaseError> {}
}
