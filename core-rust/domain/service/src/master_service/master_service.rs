use std::sync::Arc;
use model::users::Role;
use tracing::{error, warn};

use async_trait::async_trait;
use use_case::{
    AuthRepo, HashRepo, JwtRepo, MasterRespon, MasterUseCase, MasterUseCaseError, PolicyRepo, TimeSystemRepo, UserRepo, VerifyStatus
};

pub struct MasterService {
    pub user_repo: Arc<dyn UserRepo + Send + Sync>,
    pub policy_repo: Arc<dyn PolicyRepo + Send + Sync>,
    pub jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
    pub auth_repo: Arc<dyn AuthRepo + Send + Sync>,
    pub hash_repo: Arc<dyn HashRepo + Send + Sync>,
    pub time_repo: Arc<dyn TimeSystemRepo + Sync + Send>,
}
impl MasterService {
    pub fn new(
        user_repo: Arc<dyn UserRepo + Send + Sync>,
        policy_repo: Arc<dyn PolicyRepo + Send + Sync>,
        jwt_repo: Arc<dyn JwtRepo + Send + Sync>,
        auth_repo: Arc<dyn AuthRepo + Send + Sync>,
        hash_repo: Arc<dyn HashRepo + Send + Sync>,
        time_repo: Arc<dyn TimeSystemRepo + Send + Sync>,
    ) -> Self {
        Self {
            user_repo,
            policy_repo,
            jwt_repo,
            auth_repo,
            hash_repo,
            time_repo,
        }
    }
}

#[async_trait]
impl MasterUseCase for MasterService {
    async fn update_user_status(
        &self,
        order: model::jwt::TokenResponse,
        order_user_id: uuid::Uuid,
        order_role: model::users::Role,
        order_status: model::users::AccountStatus,
    ) -> Result<MasterRespon, MasterUseCaseError> {
        let order_claims = self.jwt_repo.decoder(&order.access_token)
            .await
            .map_err(|e| {
            error!(error=%e,"jwt_repo fail while master claim");
            MasterUseCaseError::JwtFail(e.to_string())
        })?;
        let opt_session_record = self
            .auth_repo
            .get_sesion_by_sess_id(&order_claims.jti)
            .await
            .map_err(|e| {
                error!(error=%e,sess_id=%order_claims.jti,"auth_repo fail while master session");
                MasterUseCaseError::AuthRepoFail(e.to_string())
            })?;
        // check opt sess_id
        let session_record = match opt_session_record {
            Some(s) => s,
            None => {
                warn!(sess_id=%order_claims.jti, "session not found (maybe TTL) -> force re-login");
                return Err(MasterUseCaseError::SessionNotFond);
            }
        };
        //hash check rt_order and rt of system
        let verify_rt_status = self
            .hash_repo
            .varify_rt_hmac_sha256_base64(&order.refresh_token, &session_record.rt_hash)
            .await
            .map_err(|e| {
                error!(error=%e,rt_plain=%order.refresh_token,"hash_repo fail while master check");
                MasterUseCaseError::HashingFail(e.to_string())
            })?;
        // check rt exp
        let now = self.time_repo.now().await;
        if now > session_record.rt_exp {
            warn!(sess_id=%session_record.session_id, "refresh master token expired -> kill session");
            let _ = self
                .auth_repo
                .kill_sesion_id(&session_record.session_id)
                .await
                .map_err(|e| {
                    error!(error=%e, "auth_repo kill fail (rt expired)");
                    MasterUseCaseError::AuthRepoFail(e.to_string())
                })?;
            return Err(MasterUseCaseError::RefreshExpired);
        }
        match verify_rt_status {
            VerifyStatus::Pass => {
                //check policy_ver when admin or master work
                    let policy_ver=self.policy_repo.get_policy_version().await.map_err(|e|{
                    error!(error=%e,"policyrepo fail while get master policy version");
                    MasterUseCaseError::PolicyRepoError(e.to_string())
                })?;

                if session_record.policy_ver != policy_ver{
                    warn!(user_id=%session_record.user_id, 
                            old=%session_record.policy_ver, 
                            now=%policy_ver, 
                            "policy changed, Kill mastersession , force re-login");
                    let _ = self
                        .auth_repo
                        .kill_sesion_id(&session_record.session_id)
                        .await
                        .map_err(|e| {
                            error!(error=%e,"auth_repo fail while on master policy mismatch");
                            MasterUseCaseError::AuthRepoFail(e.to_string())
                        })?;
                    //TODO case cant kill session
                    return Err(MasterUseCaseError::PolicyVersionMismatch);
                }else if session_record.role!= Role::Master {
                    println!("{:?}",session_record.role);
                            warn!(user_id=%session_record.user_id, 
                            old=%session_record.policy_ver, 
                            now=%policy_ver, 
                            "Operation not permitted, force re-login");
                    let _ = self
                        .auth_repo
                        .kill_sesion_id(&session_record.session_id)
                        .await
                        .map_err(|e| {
                            error!(error=%e,"auth_repo fail while on master policy mismatch");
                            MasterUseCaseError::AuthRepoFail(e.to_string())
                        })?;
                    //TODO case cant kill session
                    return Err(MasterUseCaseError::PermittedFail);
                }
                //after all check now let update
                let _ =self.user_repo.update_user_status_role(
                    order_user_id,
                    order_status,
                order_role,)
                    .await
                    .map_err(|e|{
                    error!(error=%e,"master update_user_status_role fail");
                    MasterUseCaseError::UserRepoFail(e.to_string())})?;
                let new_policy_ver =self.policy_repo
                    .bump_policy_version()
                    .await.map_err(|e|{
                        error!(error=%e,"policy_repo error while master");
                        MasterUseCaseError::PolicyRepoError(e.to_string())
                    })?;
                println!("{new_policy_ver:?}");
                Ok(MasterRespon::Update { new_policy_ver})
            },
            VerifyStatus::Fail => {
                warn!(rt_hash=%session_record.rt_hash,rt_plain=%order.refresh_token,"bad master BadRequet");
                Err(MasterUseCaseError::BadRequet)
            }
            VerifyStatus::Corrupted => {
                //TODO hendel this case
                todo!()
            }
        }
    }
}
    

