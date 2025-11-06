use async_trait::async_trait;
use deadpool_redis::{
    Config, Runtime,
    redis::{AsyncCommands, cmd},
};
use model::jwt::SessionRecord;
//use redis::AsyncCommands;
use use_case::{AuthRepo, AuthRepoError};

pub struct CashRedisService {
    pub redis_pool: deadpool_redis::Pool,
}

impl CashRedisService {
    pub fn new(params: &str) -> Result<Self, AuthRepoError> {
        let redis = Config::from_url(params);
        let redis_pool = redis
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        Ok(CashRedisService { redis_pool })
    }
}

#[async_trait]
impl AuthRepo for CashRedisService {
    async fn get_sesion_by_sess_id(
        &self,
        session_id: &str,
    ) -> Result<Option<SessionRecord>, AuthRepoError> {
        let mut con = self
            .redis_pool
            .get()
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        let val: Option<String> = con
            .get(session_id)
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        let Some(json_str) = val else {
            return Ok(None);
        };
        let session: SessionRecord = serde_json::from_str(&json_str)
            .map_err(|e| AuthRepoError::FormatError(e.to_string()))?;

        Ok(Some(session))
    }
    async fn create_session(&self, session: &SessionRecord) -> Result<(), AuthRepoError> {
        let mut con = self
            .redis_pool
            .get()
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        let json_str = serde_json::to_string(session)
            .map_err(|e| AuthRepoError::FormatError(e.to_string()))?;

        cmd("SET")
            .arg(&session.session_id)
            .arg(&json_str)
            .arg("EXAT")
            .arg(session.expires_at) // i64 epoch seconds
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;

        Ok(())
    }
    async fn kill_sesion_id(&self, session_id: &String) -> Result<bool, AuthRepoError> {
        let mut con = self
            .redis_pool
            .get()
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        let res_int: i64 = cmd("DEL")
            .arg(session_id)
            .query_async(&mut con)
            .await
            .map_err(|e| AuthRepoError::EnginFail(e.to_string()))?;
        Ok(res_int > 0)
    }
}
#[cfg(test)]
mod tests {
    use hasher::HashService;
    use model::jwt::{AuthConfig, SessionRecordBuild};
    use refresh_token::RefreshTokenService;
    use timesystem::TimeSystemService;
    use use_case::{HashRepo, RefreshRepo, TimeSystemRepo};
    use uuid::Uuid;

    use super::*;
    #[tokio::test]
    async fn redis_auth_session_roundtrip() {
        let params = "redis://127.0.0.1:6379".to_string();
        let crs = CashRedisService::new(&params).unwrap();
        let time = TimeSystemService::new();
        let now = time.now().await;
        let user_id_test = Uuid::new_v4();
        let policy_ver = 1;
        let rt_service = RefreshTokenService::new();
        let cfg = AuthConfig::new(900, 6000, 6000);
        let refresh_token =
            RefreshTokenService::gen_refresh_token_base64(&rt_service, now, cfg.refresh_ttl)
                .await
                .unwrap();
        let hasher_service = HashService::new_default([7u8; 32]);
        let rt_hash =
            HashService::hash_rt_hmac_sha256_base64(&hasher_service, &refresh_token.token_plain)
                .await
                .unwrap();

        let session = SessionRecordBuild::new(
            user_id_test,
            model::users::Role::User,
            model::users::AccountStatus::Active,
            &rt_hash,
            refresh_token.token_exp,
            now,
            policy_ver,
        )
        .cfg(cfg)
        .build();
        let res_set = crs.create_session(&session).await;
        assert!(res_set.is_ok());
        let res_get = crs
            .get_sesion_by_sess_id(&session.session_id)
            .await
            .unwrap();
        assert!(res_get.is_some());
        let session_redis = res_get.unwrap();
        assert_eq!(session_redis, session);
        let res_del = crs.kill_sesion_id(&session.session_id).await;
        assert!(res_del.is_ok());
        let res_get_after_del = crs.get_sesion_by_sess_id(&session.session_id).await;
        let opt_res = res_get_after_del.unwrap();
        assert_eq!(opt_res, None);
    }
}
