use async_trait::async_trait;
use model::Users;
use sqlx::PgPool;
use use_case::{RepoError, UserAuthRepoDto, UserRepo, UserRepoDto};
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
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserRepoDto>, RepoError> {
        let row = sqlx::query_as::<_, UserRepoDto>(
            r#"SELECT id,username,email,password_hash FROM users WHERE id=$1"#,
        )
        .bind(id)
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
    ) -> Result<Option<UserRepoDto>, RepoError> {
        let row =sqlx::query_as::<_,UserRepoDto>(
           
           r#"INSERT INTO users (username ,email , password_hash ) VALUES ($1,$2,$3) RETURNING id,username,email,password_hash"#
            ).bind(username)
            .bind(email)
            .bind(password)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(Some(row))
    }
    async fn get_users(&self) -> Result<Option<Vec<Users>>, RepoError> {
        let row = sqlx::query_as::<_,UserRepoDto>( r#"SELECT id,username , email ,password_hash FROM users"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepoError::Db(e.to_string()))?;
        let dao=row.iter().map(|repo|repo.try_into()).collect();
        Ok(Some(dao))
    }
    async fn get_password_by_username(
        &self,
        username: String,
    ) -> Result<Option<UserAuthRepoDto>, RepoError> {
        let row = sqlx::query_as::<_,UserAuthRepoDto>(
            r#"SELECT username , password_hash FROM users WHERE username=$1"#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(Some(row))
    }
    async fn create_user(&self, user: Users) -> Result<(), RepoError> {
        
        let row = sqlx::query_as::<_,UserRepoDto>(
            r#"INSERT INTO users (username ,email , password_hash ) 
            VALUES ($1,$2,$3) RETURNING id,username,email,password_hash"#
            ).bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepoError::Db(e.to_string()))?;
        Ok(())
    }
}

