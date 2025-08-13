use crate::db::user_models::User;
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;
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

impl UserRepository for SqlxUserRepo {
    async fn find_user_by_id(&self, id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User,
            r#"SELECT user_id , user_name , user_email , password_hash,created_at FROM user WHERE id=$1"#,
            id
        )
        .fetch_option(&self.pool).await
    }
    async fn create(&self, name: &str, email: &str, password: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO user (user_id,user_name,user_email,password_hash)
               VALUE ($1,$2,$3,$4)
                RETURNING user_id,user_name,user_email,password_hash"#,
            Uuid::naw_v4(),
            name,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await
    }
}
