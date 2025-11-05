use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{
        Error as PhcError, PasswordHash as PhcHash, PasswordHasher, PasswordVerifier, SaltString,
    },
};
use zeroize::Zeroizing;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq; // constant-time compare

type HmacSha256 = Hmac<Sha256>;
use async_trait::async_trait;
use model::auth_model::{PasswordHash as DomainPhc, PasswordPlain};
use rand_core::OsRng;
use use_case::{HashRepo, HasherError, VerifyStatus};

pub struct HashService {
    pub argon2: Argon2<'static>,
    pub secret: Zeroizing<[u8; 32]>,
}
impl HashService {
    pub fn new_default(secret: [u8; 32]) -> Self {
        let params = Params::new(256 * 1024, 3, 1, None).expect("valid params");
        // 256 MiB, 3 รอบ, 1 thread
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        Self {
            argon2,
            secret: Zeroizing::new(secret),
        }
    }
    pub fn new_with_params(secret: [u8; 32], m_kib: u32, t_cost: u32, p_cost: u32) -> Self {
        let params = Params::new(m_kib, t_cost, p_cost, None).expect("valid params");
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        Self {
            argon2,
            secret: Zeroizing::new(secret),
        }
    }
}

#[async_trait]
impl HashRepo for HashService {
    async fn hashing_password_argon2(
        &self,
        ps_plain: PasswordPlain,
    ) -> Result<DomainPhc, HasherError> {
        let salt = SaltString::generate(&mut OsRng);
        let phc = self
            .argon2
            .hash_password(&ps_plain.as_bytes(), &salt)
            .map_err(|e| HasherError::EnginError(e.to_string()))?;
        let res = DomainPhc::from_phc(phc.to_string())
            .map_err(|e| HasherError::FormatError(e.to_string()))?;
        Ok(res)
    }
    async fn varify_password_argon2(
        &self,
        phc: DomainPhc,
        cadidaie: PasswordPlain,
    ) -> Result<use_case::VerifyStatus, HasherError> {
        let phc_argon_type =
            PhcHash::new(&phc.phc).map_err(|e| HasherError::EnginError(e.to_string()))?;
        let opt_verify = self.argon2.verify_password(&cadidaie.0, &phc_argon_type);
        match opt_verify {
            Ok(()) => Ok(VerifyStatus::Pass),
            Err(PhcError::Password) => Ok(VerifyStatus::Fail),
            Err(_) => Ok(VerifyStatus::Corrupted),
        }
    }
    async fn hash_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &str,
    ) -> Result<String, HasherError> {
        let mut mac = HmacSha256::new_from_slice(&*self.secret)
            .map_err(|e| HasherError::EnginError(e.to_string()))?;
        mac.update(rt_plain_base64.as_bytes());
        let tag = mac.finalize().into_bytes();
        let res = URL_SAFE_NO_PAD.encode(tag);
        Ok(res)
    }
    async fn varify_rt_hmac_sha256_base64(
        &self,
        rt_plain_base64: &str,
        rt_hash_bash64: &str,
    ) -> Result<use_case::VerifyStatus, HasherError> {
        //calculet new tag
        let mut mac = HmacSha256::new_from_slice(&*self.secret)
            .map_err(|e| HasherError::EnginError(e.to_string()))?;
        mac.update(rt_plain_base64.as_bytes());
        let computed_tag = mac.finalize().into_bytes();
        //------ --------//
        let stored_tag = URL_SAFE_NO_PAD
            .decode(rt_hash_bash64.as_bytes())
            .map_err(|e| HasherError::FormatError(e.to_string()))?;
        // constant-time compare
        let res: bool = computed_tag.ct_eq(&stored_tag).into();
        Ok(if res {
            VerifyStatus::Pass
        } else {
            VerifyStatus::Fail
        })
    }
}
#[cfg(test)]
mod tests {
    use refresh_token::RefreshTokenService;
    use timesystem::TimeSystemService;
    use use_case::{RefreshRepo, TimeSystemRepo};

    use super::*;

    #[tokio::test]
    async fn password_argon2_roundtrip() {
        let svc = HashService::new_with_params([7u8; 32], 256 * 1024, 2, 1);
        // dev
        // let fast
        let plain = PasswordPlain::form_vec(b"s3cret!".to_vec());
        let phc = svc.hashing_password_argon2(plain.clone()).await.unwrap();

        assert!(matches!(
            svc.varify_password_argon2(phc.clone(), plain)
                .await
                .unwrap(),
            VerifyStatus::Pass
        ));

        let wrong = PasswordPlain::form_vec(b"nope".to_vec());
        assert!(matches!(
            svc.varify_password_argon2(phc, wrong).await.unwrap(),
            VerifyStatus::Fail
        ));
    }

    #[tokio::test]
    async fn refresh_token_hmac_base64() {
        let svc = HashService::new_default([9u8; 32]);
        let now = TimeSystemService.now().await;
        let rt_ttl = 60 as i64;
        let token_from_gen = RefreshTokenService
            .gen_refresh_token_base64(now, rt_ttl)
            .await
            .expect("refresh_token_hmac_base64 fail");
        let rt_plain_b64 = &token_from_gen.token_plain; // สมมติ plain token ที่เป็น base64url แล้ว
        let tag = svc.hash_rt_hmac_sha256_base64(rt_plain_b64).await.unwrap();

        assert!(matches!(
            svc.varify_rt_hmac_sha256_base64(rt_plain_b64, &tag)
                .await
                .unwrap(),
            VerifyStatus::Pass
        ));
        assert!(matches!(
            svc.varify_rt_hmac_sha256_base64("tampered", &tag)
                .await
                .unwrap(),
            VerifyStatus::Fail
        ));
    }
}
