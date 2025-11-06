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

#[cfg(test)]
mod tests {
    use super::*;
    use model::auth_model::PasswordHash;
    use model::users::{AccountStatus, Role, Users};
    use sqlx::PgPool;
    use uuid::Uuid;

    // ===== Helper: สร้าง PgPool จาก env =====
    async fn test_pool() -> PgPool {
        // ตั้งค่า DATABASE_URL ให้ชี้ไป DB เทสของนาย
        // ตัวอย่าง: postgres://postgres:postgres@127.0.0.1:5432/mydb_test
        let url = "postgres://myuser:mypass@localhost:5432/mydb".to_string();
        PgPool::connect(&url).await.expect("connect pg")
    }
    const SAMPLE_PHC: &str =
        "$argon2id$v=19$m=65536,t=3,p=2$c29tZXNhbHQ$hWfS8n0J0bmoG8oAqD1z0e9z7s8pW5y9b0r2x4J0v9A";

    // ===== Helper: สร้าง user สำหรับเทส =====
    async fn seed_user(repo: &SqlxUserRepo, uname: &str, email: &str) -> PasswordHash {
        let phc = PasswordHash::from_phc(SAMPLE_PHC.to_string()).expect("PHC must parse");
        repo.creat_user(uname, email, phc.clone())
            .await
            .expect("insert user");
        phc
    }

    // ===== Helper: ลบ user หลังเทส (กันข้อมูลค้าง) =====
    async fn cleanup_user(pool: &PgPool, uname: &str) {
        let _ = sqlx::query(r#"DELETE FROM users WHERE username = $1"#)
            .bind(uname)
            .execute(pool)
            .await;
    }

    fn uniq(s: &str) -> String {
        format!(
            "{}_{}",
            s,
            Uuid::new_v4().to_string().split('-').next().unwrap()
        )
    }

    #[tokio::test]
    async fn test_creat_user_and_get_by_username() {
        let pool = test_pool().await;
        let repo = SqlxUserRepo::new(pool.clone());
        let uname = uniq("donut");
        let email = format!("{}@example.com", &uname);

        // seed
        let _phc = seed_user(&repo, &uname, &email).await;

        // get_user_by_username
        let u = repo
            .get_user_by_username(&uname)
            .await
            .expect("get_user_by_username ok");
        assert_eq!(u.username, uname);
        assert_eq!(u.email, email);

        // clear
        cleanup_user(&pool, &uname).await;
    }

    #[tokio::test]
    async fn test_get_password_by_username() {
        let pool = test_pool().await;
        let repo = SqlxUserRepo::new(pool.clone());
        let uname = uniq("donut_pwd");
        let email = format!("{}@example.com", &uname);

        let phc = seed_user(&repo, &uname, &email).await;

        let got = repo.get_password_by_username(&uname).await.expect("ok");
        assert!(got.is_some());
        assert_eq!(got.unwrap(), phc);

        cleanup_user(&pool, &uname).await;
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let pool = test_pool().await;
        let repo = SqlxUserRepo::new(pool.clone());
        let uname = uniq("donut_id");
        let email = format!("{}@example.com", &uname);
        seed_user(&repo, &uname, &email).await;

        // ดึง id มาก่อน
        let id: Uuid = sqlx::query_scalar(r#"SELECT id FROM users WHERE username = $1"#)
            .bind(&uname)
            .fetch_one(&pool)
            .await
            .expect("get id");

        let got = repo.get_user_by_id(id).await.expect("ok");
        assert!(got.is_some());
        let u = got.unwrap();
        assert_eq!(u.username, uname);
        assert_eq!(u.email, email);

        cleanup_user(&pool, &uname).await;
    }

    #[tokio::test]
    async fn test_check_username_and_email() {
        let pool = test_pool().await;
        let repo = SqlxUserRepo::new(pool.clone());
        let uname = uniq("donut_exist");
        let email = format!("{}@example.com", &uname);

        // ก่อน insert → ต้องไม่มี
        let u0 = repo.check_username(&uname).await.expect("ok");
        let e0 = repo.check_email(&email).await.expect("ok");
        assert!(u0.is_none());
        assert!(e0.is_none());

        // insert
        seed_user(&repo, &uname, &email).await;

        // หลัง insert → ต้องมี
        let u1 = repo.check_username(&uname).await.expect("ok");
        let e1 = repo.check_email(&email).await.expect("ok");
        assert!(u1.is_some());
        assert!(e1.is_some());

        cleanup_user(&pool, &uname).await;
    }

    #[tokio::test]
    async fn test_update_user_status_role() {
        let pool = test_pool().await;
        let repo = SqlxUserRepo::new(pool.clone());
        let uname = uniq("donut_update");
        let email = format!("{}@example.com", &uname);
        seed_user(&repo, &uname, &email).await;

        // ดึง id
        let id: Uuid = sqlx::query_scalar(r#"SELECT id FROM users WHERE username = $1"#)
            .bind(&uname)
            .fetch_one(&pool)
            .await
            .expect("get id");

        // อัปเดต role/status
        repo.update_user_status_role(id, AccountStatus::Active, Role::Admin)
            .await
            .expect("update ok");

        // ตรวจผล
        let row: (AccountStatus, Role) =
            sqlx::query_as(r#"SELECT status, role FROM users WHERE id = $1"#)
                .bind(id)
                .fetch_one(&pool)
                .await
                .expect("fetch after update");

        assert_eq!(row.0, AccountStatus::Active);
        assert_eq!(row.1, Role::Admin);

        cleanup_user(&pool, &uname).await;
    }
}
