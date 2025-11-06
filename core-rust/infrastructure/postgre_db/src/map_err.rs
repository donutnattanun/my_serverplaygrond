use sqlx::{Error, postgres::PgDatabaseError};
use use_case::UserRepoError;

pub trait ToUserRepoError {
    fn to_user_repo_error(self) -> UserRepoError;
}
impl ToUserRepoError for Error {
    fn to_user_repo_error(self) -> UserRepoError {
        // from SQLx::Error to my enum
        match self {
            Error::Database(db_err) => {
                // error form Postgres
                if let Some(pg) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    match pg.code() {
                        "23505" => {
                            // duplicate key violation
                            let constraint = pg.constraint().unwrap_or("unknown").to_string();
                            UserRepoError::DuplicateKey(constraint)
                        }
                        _ => UserRepoError::EnginError(format!(
                            "PG error code {}: {}",
                            pg.code(),
                            pg.message()
                        )),
                    }
                } else {
                    UserRepoError::EnginError(db_err.to_string())
                }
            }
            other => UserRepoError::EnginError(other.to_string()),
        }
    }
}
