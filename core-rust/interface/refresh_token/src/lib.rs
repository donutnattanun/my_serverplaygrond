use rand::{TryRngCore, rngs::OsRng};
use use_case::{RefreshRepo, RefreshRepoError, RefreshToken};
pub struct RefreshTokenService;
use async_trait::async_trait;
#[async_trait]
impl RefreshRepo for RefreshTokenService {
    async fn gen_refresh_token_base64(
        &self,
        now: i64,
        rt_ttl: i64,
    ) -> Result<use_case::RefreshToken, RefreshRepoError> {
        let mut buff = [0u8; 32];
        OsRng
            .try_fill_bytes(&mut buff)
            .map_err(|e| RefreshRepoError::Enginfail(e.to_string()))?;
        let token_plain = URL_SAFE_NO_PAD.encode(buf);
        let token_exp = now + rt_ttl;
        Ok(RefreshToken::new(token_plain, token_exp))
    }
}
