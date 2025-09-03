use async_trait::async_trait;
use model::users;
use sqlx::PgPool;
use use_case::{RepoError, UserRepo};
use uuid::Uuid;

#[derive(Clone)]
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
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<users>, RepoError> {
        let row = sqlx::query_as!(
            users,
            r#"SELECT id,username,email,password_hash FROM users WHERE id=$1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(row)
    }
    async fn new_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<Option<users>, RepoError> {
        let row =sqlx::query_as!(
           users,
           r#"INSERT INTO users (username ,email , password_hash ) VALUES ($1,$2,$3) RETURNING id,username,email,password_hash"#,username,email,password
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(Some(row))
    }
    async fn get_users(&self) -> Result<Option<users>, RepoError> {
        let row = sqlx::query_as!(users, r#"SELECT * FROM users"#)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(row)
    }
    async fn get_password_by_username(
        &self,
        username: String,
    ) -> Result<Option<(String, String)>, RepoError> {
        let row = sqlx::query_as!(
            (String, String),
            r#"SELECT username , password_hash FROM users WHERE username=$1"#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(Some(row))
    }
}
