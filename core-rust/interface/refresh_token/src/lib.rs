use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{TryRngCore, rngs::OsRng};
use use_case::{RefreshRepo, RefreshRepoError, RefreshToken};
pub struct RefreshTokenService;
impl RefreshTokenService {
    pub fn new() -> Self {
        Self
    }
}
use async_trait::async_trait;
#[async_trait]
impl RefreshRepo for RefreshTokenService {
    async fn gen_refresh_token_base64(
        &self,
        now: i64,
        rt_ttl: u32,
    ) -> Result<use_case::RefreshToken, RefreshRepoError> {
        let mut buff = [0u8; 32];
        OsRng
            .try_fill_bytes(&mut buff)
            .map_err(|e| RefreshRepoError::Enginfail(e.to_string()))?;
        let token_plain = URL_SAFE_NO_PAD.encode(buff);
        let token_exp = now + rt_ttl as i64;
        Ok(RefreshToken::new(token_plain, token_exp))
    }
}
