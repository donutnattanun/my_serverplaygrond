#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::fmt::format;
    use std::{sync::Arc, time::Duration};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    use axum::Router;
    use deadpool_redis::Pool as RedisPool;
    use deadpool_redis::{Config as RedisConfig, Runtime};
    use http::StatusCode;
    use reqwest::Client;
    use sqlx::PgPool;
    use tower::ServiceBuilder;

    // ======= import ของโปรเจกต์นาย =======
    use crate::routes::{SharedState, routes};
    use async_trait::async_trait;
    use model::{
        auth_model::{UserLogin, UserSingup},
        jwt::TokenResponse,
    };
    use use_case::{AuthUserCase, AuthUserCaseError, LogoutResult}; // เปลี่ยนเป็น path จริง

    // ---------- Dummy Auth svc (ไม่ได้ใช้ใน /check แตะ state ให้ครบ) ----------
    fn make_token_ok() -> TokenResponse {
        TokenResponse {
            access_token: "at_dummy".into(),
            refresh_token: "rt_dummy".into(),
            expires_in: 900,
            token_type: "Bearer".into(),
        }
    }
    fn make_token_refresh_res_ok() -> TokenResponse {
        TokenResponse {
            access_token: "at_dummy_rh".into(),
            refresh_token: "rt_dummy_rh".into(),
            expires_in: 900,
            token_type: "Bearer".into(),
        }
    }

    struct DummyAuthSvc;
    #[async_trait]
    impl AuthUserCase for DummyAuthSvc {
        async fn login(&self, order: UserLogin) -> Result<TokenResponse, AuthUserCaseError> {
            if order.username == "donut" {
                Ok(TokenResponse {
                    access_token: "at_dummy".into(),
                    refresh_token: "rt_dummy".into(),
                    expires_in: 900,
                    token_type: "Bearer".into(),
                })
            } else {
                Err(AuthUserCaseError::BadRequet)
            }
        }
        async fn logout(
            &self,
            order: TokenResponse,
        ) -> Result<use_case::LogoutResult, AuthUserCaseError> {
            if order == make_token_refresh_res_ok() {
                Ok(LogoutResult::SessionTerminated)
            } else {
                Err(AuthUserCaseError::BadRequet)
            }
        }
        async fn singup(&self, order: UserSingup) -> Result<(), AuthUserCaseError> {
            Ok(())
        }
        async fn refresh_token(
            &self,
            order: TokenResponse,
        ) -> Result<TokenResponse, AuthUserCaseError> {
            let ok_token = make_token_ok();
            if ok_token == order {
                let res = make_token_refresh_res_ok();
                Ok(res)
            } else {
                Err(AuthUserCaseError::BadRequet)
            }
        }
    }

    // ---------- helper: start server บนพอร์ตสุ่ม ----------
    async fn spawn_server(app: Router) -> (String, JoinHandle<()>) {
        // bind 127.0.0.1:0 เอาพอร์ตสุ่ม
        let listener = TcpListener::bind(("127.0.0.1", 0)).await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        let hendle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app.into_make_service()).await {
                eprintln!("server error: {e}");
            }
        });

        (url, hendle)
    }
    async fn read_json<T: serde::de::DeserializeOwned>(
        res: reqwest::Response,
    ) -> (StatusCode, String, ApiResp<T>) {
        let status = res.status();
        let raw = res.text().await.expect("read body");
        println!("raw response = {}", raw);
        let val =
            serde_json::from_str::<ApiResp<T>>(&raw).expect("schema must include message+data");
        (status, raw, val)
    }

    #[derive(serde::Deserialize, Debug)]
    struct ApiResp<T> {
        status: String,
        message: Option<String>,
        data: Option<T>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct CheckServices {
        database: bool,
        redis: bool,
    }

    #[derive(serde::Deserialize, Debug)]
    struct CheckResp {
        status: String,
        services: CheckServices,
        timestamp: Option<String>,
    }
    async fn read_plain<T: serde::de::DeserializeOwned>(
        res: reqwest::Response,
    ) -> (StatusCode, String, T) {
        let status = res.status();
        let raw = res.text().await.expect("read body");
        println!("raw response = {}", raw);
        let parsed = serde_json::from_str::<T>(&raw).expect("parse CheckResp");
        (status, raw, parsed)
    }

    #[tokio::test]
    async fn e2e_check_endpoint_ok() {
        // ---------- ENV  ----------
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://myuser:mypass@localhost:5432/mydb".to_string());
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        // ---------- PgPool จริง (SELECT 1 ใช้ได้เลย ไม่ต้อง migrate) ----------
        let pg_pool = PgPool::connect(&database_url).await.expect("pg connect");
        // warmup
        sqlx::query("SELECT 1")
            .fetch_one(&pg_pool)
            .await
            .expect("pg ready");

        // ---------- Redis pool จริง ----------
        let cfg = RedisConfig::from_url(redis_url);
        let redis_pool: RedisPool = cfg.create_pool(Some(Runtime::Tokio1)).expect("redis pool");
        {
            let mut conn = redis_pool.get().await.expect("redis get");
            use deadpool_redis::redis::AsyncCommands;
            let _: () = conn.ping().await.expect("redis ping");
        }

        // ---------- state ----------
        let state = SharedState {
            svc: Arc::new(DummyAuthSvc),
            pg_pool: pg_pool.clone(),
            redis_pool: redis_pool.clone(),
        };

        // ---------- app ----------
        let app = routes(state);

        // ---------- run server ----------
        let (base, _hendle) = spawn_server(app).await;
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        // ---------- call /check ----------

        // 1) /check
        let res_check = client.get(format!("{}/check", base)).send().await.unwrap();
        let (st_check, _raw, resq) = read_plain::<CheckResp>(res_check).await;
        assert_eq!(st_check, StatusCode::OK);
        assert_eq!(resq.status, "ok");
        assert!(resq.services.database);
        assert!(resq.services.redis);
        assert!(resq.timestamp.is_some());

        // 2) /auth/login
        let (st_login, _raw, login_resp) = read_json::<TokenResponse>(
            client
                .post(format!("{}/auth/login", base))
                .json(&serde_json::json!({ "username":"donut", "password":"1234" }))
                .send()
                .await
                .unwrap(),
        )
        .await;

        assert_eq!(st_login, StatusCode::OK);
        assert_eq!(login_resp.message, Some("Login successful".to_string()));
        let token = login_resp.data.unwrap(); // ได้ TokenResponse แบบ type-safe

        // 3) /auth/refresh
        let (st_refresh, _raw, refresh_resp) = read_json::<TokenResponse>(
            client
                .post(format!("{}/auth/refresh", base))
                .json(&token) // ส่ง struct เป็น JSON ได้ตรง ๆ
                .send()
                .await
                .unwrap(),
        )
        .await;

        assert_eq!(st_refresh, StatusCode::OK);
        let refreshed = refresh_resp.data.unwrap();

        // 4) /auth/logout
        let (st_logout, _, _) = read_json::<TokenResponse>(
            client
                .post(format!("{}/auth/logout", base))
                .json(&refreshed) // ใช้ token ล่าสุด
                .send()
                .await
                .unwrap(),
        )
        .await;

        assert_eq!(st_logout, StatusCode::OK);
    }
}
