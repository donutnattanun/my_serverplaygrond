use std::sync::Arc;
use use_case::{AuthRepo, AuthUserCase, HashRepo, UserRepo,AuthUserCaseError};
use model::{auth_model::UserLogin,jwt_key_model::jwt::{TokenResponse}};
pub struct AuthService {
    pub auth_repo: Arc<AuthRepo + Send + Sync>,
    pub hash_repo: Arc<HashRepo + Send + Sync>,
    pub user_repo: Arc<UserRepo + Send + Sync>,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<AuthRepo + Send + Sync>,
        hash_repo: Arc<HashRepo + Send + Sync>,
        user_repo: Arc<UserRepo + Send + Sync>,

    )->Self{Self{auth_repo,hash_repo,user_repo}}
    
}
#[async_trait::async_trait]
impl AuthUserCase for AuthService {
    async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError> {
        let phc = self.hash_repo.hashing(order).await.map_err(|e|AuthUserCaseError::HashingFail(e.to_string()))?;
        let phc_db =self.user_repo.get_password_by_username(&order.username).await.map_err(|e|AuthUserCaseError::DbFail(e.to_string()))?;
        if let Some(phc_db)=phc_db{
            let varify=self.hash_repo.varify(phc_db, phc).await.map_err(|e|AuthUserCaseError::HashingFail(e.to_string())?;
        }else {
            return AuthUserCaseError::InvalidRequet;
        }


        
    }
}
