use crate::users_model::{AcconutStatus, Role};
use uuid::Uuid;

#[derive(Debug)]
pub struct TokenResponse {
    pub access_token: String,  // at_xxx (opaque)
    pub refresh_token: String, // rt_xxx (opaque)
    pub expires_in: u32,       // seconds (ของ access)
    pub token_type: String,    // "Bearer"
}

// ========== session ที่จะเก็บใน Redis ==========

#[derive(Debug)]
pub struct SessionRecord {
    pub session_id: String, // sess_xxx (ช่วย logout per device)
    pub user_id: Uuid,
    pub role: Role,
    pub status: AcconutStatus,
    pub scope: Vec<String>,     // เช่น ["users:read","users:approve"]
    pub created_at: i64,        // epoch
    pub expires_at: i64,        // epoch ของ access/refresh (แล้วแต่ key)
    pub device: Option<String>, //for test real maht have
    pub ip: Option<String>,     //for test real maht have
}

#[derive(Debug)]
pub struct AuthConfig {
    pub access_ttl: u32,  // e.g. 900 (15m)
    pub refresh_ttl: u32, // e.g. 2592000 (30d)
}
