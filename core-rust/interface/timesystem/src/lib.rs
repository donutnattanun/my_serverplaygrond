use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};
use use_case::TimeSystemRepo;
pub struct TimeSystemService;
impl TimeSystemService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TimeSystemRepo for TimeSystemService {
    async fn now(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default() //seft for under flow
            .as_secs() as i64
    }
}
