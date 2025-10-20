use thiserror::Error;
use zeroize::Zeroizing;
#[derive(Debug)]
pub struct PasswordHash {
    pub phc: String,
}
impl PasswordHash {
    pub fn from_phc(phc: String) -> Result<Self, PasswordError> {
        //check real hash logic //
        if !phc.starts_with("argon2") {
            return Err(PasswordError::Policy);
        }
        Ok(Self { phc })
    }
}
#[derive(Debug)]
pub struct PasswordPlain(pub Zeroizing<Vec<u8>>);

impl PasswordPlain {
    pub fn form_vec(v: Vec<u8>) -> Self {
        PasswordPlain(Zeroizing::new(v))
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("policy violation")]
    Policy,
    #[error("hashing failed")]
    HashingFailed,
    #[error("verify failed")]
    VerifyFailed,
}
