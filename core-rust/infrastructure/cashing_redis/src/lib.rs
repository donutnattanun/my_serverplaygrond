mod test_cashing_redis;
//-----//
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
