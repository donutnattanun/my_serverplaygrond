use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use model::{jwt::Claims, jwt::SessionRecord};
use serde::{Deserialize, Serialize};
use std::convert::From;
use use_case::{JwtRepo, JwtRepoError};

pub struct JwtCfg {
    pub kid: Option<String>,
    pub alg: Algorithm,
}
impl JwtCfg {
    pub fn new_default() -> Self {
        Self {
            kid: Some("DEMO-V.1".to_string()),
            alg: Algorithm::EdDSA,
        }
    }
}

pub struct JwtService {
    pub decord_key: DecodingKey,
    pub encode_key: EncodingKey,
    pub jwtcfg: JwtCfg,
}
impl JwtService {
    pub fn new(cfg: JwtCfg, de_key: DecodingKey, en_key: EncodingKey) -> Self {
        Self {
            decord_key: de_key,
            encode_key: en_key,
            jwtcfg: cfg,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub iss: String,
    pub sub: String,
    pub jti: String,
    pub iat: i64,
    pub exp: i64,
    pub policy_ver: u32,
}

impl From<Claims> for JwtClaims {
    fn from(c: Claims) -> Self {
        JwtClaims {
            iss: c.iss,
            sub: c.sub,
            jti: c.jti,
            iat: c.iat,
            exp: c.exp,
            policy_ver: c.policy_ver,
        }
    }
}

impl From<JwtClaims> for Claims {
    fn from(c: JwtClaims) -> Self {
        Claims {
            iss: c.iss,
            sub: c.sub,
            jti: c.jti,
            iat: c.iat,
            exp: c.exp,
            policy_ver: c.policy_ver,
        }
    }
}
#[async_trait]
impl JwtRepo for JwtService {
    async fn encoder(
        &self,
        session: &SessionRecord,
        at_ttl: u32,
        now: i64,
    ) -> Result<(String, i64), JwtRepoError> {
        let claims = Claims::new(
            //TODO fix paramiter for support orter sup
            //now for demo is for one sup
            "go.gateway".to_string(),
            session.session_id.to_string(),
            now,
            at_ttl as i64,
            session.policy_ver,
        );
        let header = Header {
            kid: self.jwtcfg.kid.clone(),
            alg: self.jwtcfg.alg,
            ..Default::default()
        };
        let exp_res = claims.exp;
        let jwtclaims: JwtClaims = claims.into();
        let token = encode(&header, &jwtclaims, &self.encode_key)
            .map_err(|e| JwtRepoError::EnginFail(e.to_string()))?;
        Ok((token, exp_res))
    }
    async fn decoder(&self, token: &str) -> Result<Claims, JwtRepoError> {
        //TODO make Validation set to cfg
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&["rust.auth.server"]);
        validation.sub = Some("go.gateway".to_string());
        let token_data = decode::<JwtClaims>(&token, &self.decord_key, &validation)
            .map_err(|e| JwtRepoError::EnginFail(e.to_string()))?;
        let tokenclaims = token_data.claims;
        let res: Claims = tokenclaims.into();
        Ok(res)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
