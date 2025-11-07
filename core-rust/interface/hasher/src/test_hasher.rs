#[cfg(test)]
mod tests {
    use crate::*;
    use refresh_token::RefreshTokenService;
    use timesystem::TimeSystemService;
    use use_case::{RefreshRepo, TimeSystemRepo};

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
        let rt_ttl = 60 as u32;
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
