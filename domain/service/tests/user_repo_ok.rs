use service::*;
use use_case::*;
use uuid::Uuid;

#[cfg(test)]
mod test {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use use_case::UserRepoDto;
    fn user_mock_by_id(id: Uuid) -> use_case::UserRepoDto {
        use_case::UserRepoDto {
            id: id,
            username: "donut".into(),
            email: "donut@example.com".into(),
            password_hash: "1234".into(),
        }
    }

    struct RepoOk;
    #[async_trait]
    impl UserRepo for RepoOk {
        async fn find_user_by_id(
            &self,
            id: Uuid,
        ) -> Result<Option<use_case::UserRepoDto>, use_case::RepoError> {
            Ok(Some(user_mock_by_id(id)))
        }
        async fn new_user(
            &self,
            username: String,
            email: String,
            password: String,
        ) -> Result<Option<UserRepoDto>, use_case::RepoError> {
            unimplemented!()
        }
        async fn get_users(&self) -> Result<Option<Vec<UserRepoDto>>, use_case::RepoError> {
            unimplemented!()
        }
        async fn get_password_by_username(
            &self,
            username: String,
        ) -> Result<Option<use_case::UserAuthRepoDto>, use_case::RepoError> {
            match username.as_str() {
                "donut" => Ok(Some(UserAuthRepoDto {
                    username: "donut".into(),
                    password_hash: "12345".into(),
                })),
                "donut1234" => Ok(Some(UserAuthRepoDto {
                    username: "donut1234".into(),
                    password_hash: "67890".into(),
                })),
                _ => Err(RepoError::Db("boom".into())),
            }
        }
    }
    #[tokio::test]
    async fn test_login_ok() {
        let repo = Arc::new(RepoOk);
        let svc = UserService::new(repo);
        let got = svc.user_login("donut".into(), "12345".into()).await;
        assert!(got.is_ok())
    }
    #[tokio::test]
    async fn test_login_wrong_password() {
        let repo = Arc::new(RepoOk);
        let svc = UserService::new(repo);
        let got = svc.user_login("donut".into(), "1234".into()).await;
        assert!(matches!(got, Err(AuthError::Invalid)));
    }
    #[tokio::test]
    async fn test_login_not_found() {
        let repo = Arc::new(RepoOk);
        let svc = UserService::new(repo);
        let got = svc.user_login("notfound".into(), "123445".into()).await;
        assert!(matches!(got, Err(AuthError::Db(_))));
    }
}
