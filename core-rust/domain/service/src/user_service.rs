use std::sync::Arc;

use tracing::{error, info, warn};
use use_case::{
    AuthError, ServiceError, UserLoginOrder, UserRepo, UserUseCase, UserUseCaseDto, Valid,
};
use uuid::Uuid;

pub struct UserService {
    pub repo: Arc<dyn UserRepo + Send + Sync>,
}

impl UserService {
    pub fn new(new_repo: Arc<dyn UserRepo + Send + Sync>) -> UserService {
        Self { repo: new_repo }
    }
}
#[async_trait::async_trait]
impl UserUseCase for UserService {
    async fn user_login(&self, req: Valid<UserLoginOrder>) -> Result<(), use_case::AuthError> {
        let Valid(order) = req;
        info!(username=%order.username,"login attempt");
        let row = self
            .repo
            .get_password_by_username(order.username.clone())
            .await
            .map_err(|e| {
                error!(error=%e,"login falid: db error");
                AuthError::Db(e.to_string())
            })?;
        let Some(row) = row else {
            warn!(username=%order.username,"login falid: NotFond");
            return Err(AuthError::Invalid);
        };
        if row.password_hash != order.password {
            warn!(username=%order.username,"login falid: wrong password");
            return Err(AuthError::Invalid);
        };

        Ok(())
    }
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), use_case::ServiceError> {
        let row = self
            .repo
            .new_user(username, email, password)
            .await
            .map_err(|e| ServiceError::Db(e.to_string()))?
            .ok_or(ServiceError::NotFond)?;
        println!("{row:?}");
        Ok(())
    }
    async fn get_users(&self) -> Result<Vec<use_case::UserUseCaseDto>, ServiceError> {
        let row = self
            .repo
            .get_users()
            .await
            .map_err(|e| ServiceError::Db(e.to_string()))?
            .ok_or(ServiceError::NotFond)?;
        let dtos: Vec<UserUseCaseDto> = row
            .into_iter()
            .map(|repo| UserUseCaseDto::from(repo))
            .collect();
        Ok(dtos)
    }
    async fn get_user(&self, id: String) -> Result<UserUseCaseDto, ServiceError> {
        let row = self
            .repo
            .find_user_by_id(Uuid::parse_str(&id).unwrap())
            .await
            .map_err(|e| ServiceError::Db(e.to_string()))?
            .ok_or(ServiceError::NotFond)?;
        //debug
        println!("is id = {id:?}");
        Ok(UserUseCaseDto::from(row))
    }
}
