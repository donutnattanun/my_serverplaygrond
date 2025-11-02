use crate::users_model::{AcconutStatus, Role};
use uuid::Uuid;

#[derive(Debug)]
pub struct TokenResponse {
    pub access_token: String,  // at_xxx (opaque)
    pub refresh_token: String, // rt_xxx (opaque)
    pub expires_in: u32,       // seconds of access for font check refech
    pub token_type: String,    // "Bearer"
}
impl TokenResponse {
    pub fn new(at: String, rt_plain: &String, exp_sec: u32) -> Self {
        //TODO validation
        Self {
            access_token: at,
            refresh_token: rt_plain.to_string(),
            expires_in: exp_sec,
            token_type: "Bearer".to_string(),
        }
    }
}
// ========== claims for gen token ======= \\
#[derive(Debug)]
pub struct Claims {
    pub iss: String, //owner server token --this case is rust.aurh.server--
    pub sub: String, //sup server--this case is follow struct jwt cfg --
    pub jti: String, //metadata --this case is  format!("sess_{}", Uuid::new_v4());
    pub iat: i64,    //time epoch
    pub exp: i64,    //time exp only sec !!not epoch for validate decoder
    pub policy_ver: u32, //version of token use when update user status !! this is for sub check
                     //aurh check in auth store and cfg
}
impl Claims {
    pub fn new(sub: String, jti: String, at_ttl: i64, now: i64, policy_ver: u32) -> Self {
        Self {
            iss: "rust.auth.server".to_string(),
            sub,
            jti,
            iat: now,
            exp: at_ttl,
            policy_ver,
        }
    }
}
// ========== session For Redis ========== \\

#[derive(Debug)]
pub struct SessionRecord {
    pub session_id: String, // sess_xxx (ช่วย logout per device)
    pub user_id: Uuid,
    pub role: Role,
    pub status: AcconutStatus,
    pub created_at: i64,        // epoch
    pub expires_at: i64,        // epoch ของ access/refresh (แล้วแต่ key)
    pub rt_hash: String,        // for check refresh key
    pub rt_exp: i64,            // exp rt only epoch!!!!
    pub device: Option<String>, //for test real maht have
    pub ip: Option<String>,     //for test real maht have
    pub policy_ver: u32,        //form policy_ver at AuthConfig Struct
}
#[derive(Debug)]
pub struct SessionRecordBuild {
    user_id: Uuid,
    role: Role,
    status: AcconutStatus,
    rt_hash: String,
    rt_exp: i64,
    device: Option<String>,
    ip: Option<String>,
    now: i64,
    cfg: AuthConfig,
}
impl SessionRecordBuild {
    pub fn new(
        user_id: Uuid,
        role: Role,
        status: AcconutStatus,
        rt_hash: &String,
        rt_exp: i64,
        now: i64,
    ) -> Self {
        Self {
            user_id,
            role,
            status,
            rt_hash: rt_hash.to_string(),
            rt_exp,
            device: None,
            ip: None,
            now,
            cfg: AuthConfig::new(900, 2592000, 86400, 0), //default
        }
    }
    pub fn device(mut self, d: String) -> Self {
        self.device = Some(d);
        self
    }
    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }
    pub fn cfg(mut self, cfg: AuthConfig) -> Self {
        self.cfg = cfg;
        self
    }
    pub fn build(self) -> SessionRecord {
        let session_id = format!("sess_{}", Uuid::new_v4());
        let created_at = self.now;
        let expires_at = created_at + self.cfg.sesion_ttl as i64;
        SessionRecord {
            user_id: self.user_id,
            session_id,
            role: self.role,
            status: self.status,
            created_at,
            expires_at,
            rt_hash: self.rt_hash,
            rt_exp: self.rt_exp,
            device: self.device,
            policy_ver: self.cfg.policy_version,
            ip: self.ip,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub access_ttl: u32,  // e.g. 900 (15m)
    pub refresh_ttl: u32, // e.g. 2592000 (30d)
    pub sesion_ttl: u32,
    pub policy_version: u32, // for master and admin reworke ro ban user update sync
}
impl AuthConfig {
    pub fn new(at_ttl: u32, rt_ttl: u32, ss_ttl: u32, policy_ver: u32) -> Self {
        Self {
            access_ttl: at_ttl,
            refresh_ttl: rt_ttl,
            sesion_ttl: ss_ttl,
            policy_version: policy_ver,
        }
    }
}
