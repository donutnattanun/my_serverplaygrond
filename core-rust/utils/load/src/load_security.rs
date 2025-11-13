use jsonwebtoken::{DecodingKey, EncodingKey};
use std::fs;
use thiserror::Error;
#[derive(Debug)]
pub struct Security {
    pub kid: String,
    pub enc_pem: EncodingKey,
    pub dec_pem: DecodingKey,
    pub hmac_key: [u8; 32],
}
impl Security {
    pub fn form_load_file(
        kid: impl Into<String>,
        priv_path: &str,
        pub_path: &str,
        hmac_key_path: &str,
    ) -> Result<Self, SecurityError> {
        let priv_pem = fs::read_to_string(priv_path)
            .map_err(|e| SecurityError::FileReadError(e.to_string()))?;
        println!("LOAD priv_pem : success");
        let pub_pem = fs::read_to_string(pub_path)
            .map_err(|e| SecurityError::FileReadError(e.to_string()))?;
        println!("LOAD priv_pem : success");
        let enc_pem = EncodingKey::from_ed_pem(priv_pem.as_bytes())
            .map_err(|e| SecurityError::KeyBuildError(e.to_string()))?;
        let dec_pem = DecodingKey::from_ed_pem(pub_pem.as_bytes())
            .map_err(|e| SecurityError::KeyBuildError(e.to_string()))?;
        let key = load_hmac_key(hmac_key_path)?;
        println!("Load security  : successfull");

        Ok(Security {
            kid: kid.into(),
            enc_pem,
            dec_pem,
            hmac_key: key,
        })
    }
}
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("std::fs fail :{0}")]
    FileReadError(String),
    #[error("Build Key fail :{0}")]
    KeyBuildError(String),
    #[error("decore base64 key error : {0}")]
    Base64Error(String),
    #[error("FormatKeyError:{0}")]
    FormatBase64Error(String),
}

use base64::{Engine, engine::general_purpose};
pub fn load_hmac_key(path: &str) -> Result<[u8; 32], SecurityError> {
    let raw = fs::read_to_string(path).map_err(|e| SecurityError::Base64Error(e.to_string()))?;
    let raw = raw.trim();

    let key = general_purpose::STANDARD
        .decode(raw)
        .map_err(|e| SecurityError::Base64Error(e.to_string()))?;

    if key.len() < 32 {
        return Err(SecurityError::FormatBase64Error("too short".to_string()));
    }

    Ok(key.try_into().unwrap())
}
