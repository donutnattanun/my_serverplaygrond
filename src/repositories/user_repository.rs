use crate::db::user_models::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user_by_id(&self, id: Uuid) -> sqlx::Result<Option<User>>;
    async fn create(&self, name: &str, email: &str, password: &str) -> sqlx::Result<Option<User>>;
}
