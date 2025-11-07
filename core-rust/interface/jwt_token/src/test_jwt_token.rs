#[cfg(test)]
mod tests {
    use crate::*;
    use ed25519_dalek::SigningKey;
    use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
    use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
    use model::jwt::{AuthConfig, SessionRecordBuild};
    use rand_core::OsRng;
    use timesystem::TimeSystemService;
    use use_case::TimeSystemRepo;
    #[tokio::test]
    async fn jwt_encoed_decode_ok() {
        //gen for test
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let priv_pem = signing_key.to_pkcs8_pem(LineEnding::LF).unwrap();
        let pub_pem = verifying_key.to_public_key_pem(LineEnding::LF).unwrap();
        let de_key = DecodingKey::from_ed_pem(pub_pem.as_bytes()).expect("DecodingKey fail");
        let en_key = EncodingKey::from_ed_pem(priv_pem.as_bytes()).expect("EncodingKey fail");
        let cfg = JwtCfg::new_default();
        let svc = JwtService::new(cfg, de_key, en_key);
        let at_ttl = 600 as u32;
        let now = TimeSystemService.now().await;
        let user_id = uuid::Uuid::new_v4();
        let auth_cfg = AuthConfig::new(at_ttl, 60000, 60000);
        let rt_hash = "fack".to_string();
        let session = SessionRecordBuild::new(
            user_id,
            model::users::Role::User,
            model::users::AccountStatus::Active,
            &rt_hash,
            now + auth_cfg.refresh_ttl as i64,
            now,
            1 as u32,
        )
        .cfg(auth_cfg)
        .build();
        let token = svc.encoder(&session, at_ttl, now).await;
        assert!(token.is_ok(), "expected Ok(...), got: {:?}", token);
        let token_exp = now + at_ttl as i64;
        let token = token.unwrap();
        assert_eq!(token.1, token_exp);
        let res = svc.decoder(&token.0).await;
        assert!(res.is_ok(), "expected Ok(...), got: {:?}", res);
        let claims = res.unwrap();
        assert_eq!(claims.iss, String::from("rust.auth.server"));
        assert_eq!(claims.sub, String::from("go.gateway"));
        assert_eq!(claims.policy_ver, 1);
        assert_eq!(claims.iat, now);
        assert_eq!(claims.exp, now + at_ttl as i64);
        assert_eq!(claims.jti, session.session_id);
    }
}
