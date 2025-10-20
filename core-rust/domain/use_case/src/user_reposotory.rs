use crate::{RepoError, UserAuthRepoDto, UserRepoDto};
use model::Users;
use uuid::Uuid;
#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    //lagary tair
    //todo ! refactor to chang return Users type
    //for ! use model sruct is universel
    async fn find_user_by_id(&self, id: Uuid) -> Result<Users, RepoError>;
    async fn new_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<Option<UserRepoDto>, RepoError>;
    async fn get_users(&self) -> Result<Option<Vec<UserRepoDto>>, RepoError>;
    async fn get_password_by_username(
        &self,
        username: String,
    ) -> Result<Option<UserAuthRepoDto>, RepoError>;
    //soft refactor//
    async fn create_user(&self, user: Users) -> Result<(), RepoError>;
    async fn list_users(&self) -> Result<Vec<Users>, RepoError>;
}
