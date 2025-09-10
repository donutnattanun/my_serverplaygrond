use std::sync::Arc;

use use_case::{AuthError, ServiceError, UserRepo, UserUseCase, UserUseCaseDto};
use uuid::Uuid;

pub struct UserService {
    pub repo: Arc<dyn UserRepo + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepo + Send + Sync>) -> UserService {
        Self { repo: repo }
    }
}
#[async_trait::async_trait]
impl UserUseCase for UserService {
    async fn user_login(
        &self,
        username: String,
        password: String,
    ) -> Result<(), use_case::AuthError> {
        let row = self
            .repo
            .get_password_by_username(username)
            .await
            .map_err(|e| AuthError::Db(e.to_string()))?;
        //debug
        println!("{row:?}");

        let Some(row) = row else {
            return Err(AuthError::Invalid);
        };
        if row.password_hash != password {
            println!("{row:?}==?{password:?}");
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
