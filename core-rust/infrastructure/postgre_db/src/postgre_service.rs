use crate::{
    map_err::ToUserRepoError,
    postgre_dto::{PasswordHashDto, UsersRowDto},
};
use async_trait::async_trait;
use model::users::Users;
use sqlx::PgPool;
use use_case::{UserRepo, UserRepoError};

pub struct SqlxUserRepo {
    pub pool: PgPool,
}
impl SqlxUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for SqlxUserRepo {
    async fn get_password_by_username(
        &self,
        username: &str,
    ) -> Result<Option<model::auth_model::PasswordHash>, UserRepoError> {
        let row = sqlx::query_as::<_, PasswordHashDto>(
            r#"SELECT password_hash FROM users WHERE username=$1"#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_user_repo_error())?;
        match row {
            Some(v) => Ok(Some(v.try_into()?)),
            None => Ok(None),
        }
    }
    async fn get_user_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<model::users::Users>, UserRepoError> {
        let row = sqlx::query_as::<_, Users>(
            r#"SELECT id,username,email,role,status FROM users WHERE id=$1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_user_repo_error())?;
        Ok(row)
    }
    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<model::users::Users, UserRepoError> {
        let row = sqlx::query_as::<_, Users>(
            r#"SELECT id,username,email,role,status FROM users WHERE username=$1"#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_user_repo_error())?;
        match row {
            Some(dto) => Ok(dto),
            None => Err(UserRepoError::NotFound),
        }
    }
    async fn creat_user(
        &self,
        username: &str,
        email: &str,
        passwordhash: model::auth_model::PasswordHash,
    ) -> Result<(), UserRepoError> {
        let user_row = UsersRowDto::make_user_row_default(username, email, passwordhash);
        sqlx::query(
            r#"INSERT INTO users (username ,email, password_hash,role,status)VALUES ($1,$2,$3,$4,$5)"#,
                )
            .bind(user_row.username)
            .bind(user_row.email)
            .bind(user_row.password_hash)
            .bind(user_row.role)
            .bind(user_row.status)
            .execute(&self.pool)
            .await
            .map_err(|e|e.to_user_repo_error())?;
        Ok(())
    }
    async fn list_user(&self) -> Result<Vec<model::users::Users>, UserRepoError> {
        unimplemented!()
    }
    async fn check_username(&self, username: &str) -> Result<Option<()>, UserRepoError> {
        let exists: (bool,) =
            sqlx::query_as(r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"#)
                .bind(username)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_user_repo_error())?;

        if exists.0 { Ok(Some(())) } else { Ok(None) }
    }
    async fn check_email(&self, email: &str) -> Result<Option<()>, UserRepoError> {
        let exists: (bool,) =
            sqlx::query_as(r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#)
                .bind(email)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_user_repo_error())?;

        if exists.0 { Ok(Some(())) } else { Ok(None) }
    }

    async fn update_user_status_role(
        &self,
        user_id: uuid::Uuid,
        user_status: model::users::AccountStatus,
        user_role: model::users::Role,
    ) -> Result<(), UserRepoError> {
        let result = sqlx::query(
            r#"UPDATE users
           SET role = $1,status =$2
           WHERE id = $3"#,
        )
        .bind(user_role)
        .bind(user_status)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_user_repo_error())?;

        if result.rows_affected() == 0 {
            return Err(UserRepoError::NotFound);
        }
        Ok(())
    }
}
