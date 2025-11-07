#[cfg(test)]
mod tests {
    use crate::postgre_service::*;
    use model::auth_model::PasswordHash;
    use model::users::{AccountStatus, Role, Users};
    use sqlx::PgPool;
    use use_case::UserRepo;
    use uuid::Uuid;

    // ===== Helper: =====
    async fn test_pool() -> PgPool {
        // let on db //
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
