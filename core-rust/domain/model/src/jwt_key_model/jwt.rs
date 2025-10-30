use crate::users_model::{AcconutStatus, Role};
use uuid::Uuid;

#[derive(Debug)]
pub struct TokenResponse {
    pub access_token: String,  // at_xxx (opaque)
    pub refresh_token: String, // rt_xxx (opaque)
    pub expires_in: u32,       // seconds (ของ access)
    pub token_type: String,    // "Bearer"
}
impl TokenResponse {
    pub fn new(at: String, rt: String, exp: u32, t_type: String) -> Self {
        Self {
            access_token: at,
            refresh_token: rt,
            expires_in: exp,
            token_type: t_type,
        }
    }
}
// ========== claims for gen token ======= \\
#[derive(Debug)]
pub struct Claims {
    iss: String, //owner server  token
    sub: String, //sup server
    jti: String, //metadata
    iat: i64,    //time
    exp: i64,    //time exp
}
impl Claims {
    pub fn new(sub: String, jti: String, at_ttl: i64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default() //seft for under flow
            .as_secs() as i64;
        let exp = now + at_ttl;
        Self {
            iss: "rust.auth.server".to_string(),
            sub,
            jti,
            iat: now,
            exp,
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
    pub rt_exp: i64,            // exp rt
    pub device: Option<String>, //for test real maht have
    pub ip: Option<String>,     //for test real maht have
}
#[derive(Debug)]
pub struct SessionRecordBuild {
    user_id: Uuid,
    role: Role,
    status: AcconutStatus,
    rt_hash: String,
    device: Option<String>,
    ip: Option<String>,
    now: i64,
    cfg: AuthConfig,
}
impl SessionRecordBuild {
    pub fn new(user_id: Uuid, role: Role, status: AcconutStatus, rt_hash: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        Self {
            user_id,
            role,
            status,
            rt_hash,
            device: None,
            ip: None,
            now: {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default() //seft for under flow
                    .as_secs() as i64
            },
            cfg: AuthConfig::new(900, 2592000, 86400), //default
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
        let rt_exp = created_at + self.cfg.refresh_ttl as i64;
        SessionRecord {
            user_id: self.user_id,
            session_id,
            role: self.role,
            status: self.status,
            created_at,
            expires_at,
            rt_hash: self.rt_hash,
            rt_exp,
            device: self.device,
            ip: self.ip,
        }
    }
}

#[derive(Debug)]
pub struct AuthConfig {
    pub access_ttl: u32,  // e.g. 900 (15m)
    pub refresh_ttl: u32, // e.g. 2592000 (30d)
    pub sesion_ttl: u32,
}
impl AuthConfig {
    pub fn new(at_ttl: u32, rt_ttl: u32, ss_ttl: u32) -> Self {
        Self {
            access_ttl: at_ttl,
            refresh_ttl: rt_ttl,
            sesion_ttl: ss_ttl,
        }
    }
}
