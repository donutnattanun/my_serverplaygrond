#[async_trait::async_trait]
pub trait TimeSystemRepo: Send + Sync {
    async fn now(&self) -> i64;
}
