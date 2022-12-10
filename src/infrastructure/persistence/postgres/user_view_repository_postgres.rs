use sqlx::{PgPool, Row};
use crate::domain::{UserCredentialsView, UserEmail, UserViewRepositoryError, UserViewRepositoryInterface};
use async_trait::async_trait;
use sqlx::postgres::PgRow;

pub struct UserViewPostgresRepository<'a>{
    pub pool: &'a PgPool,
}

#[async_trait]
impl UserViewRepositoryInterface for UserViewPostgresRepository<'_> {
    #[tracing::instrument(
    name = "Adding User Credentials View to PostgreDB",
    skip(self)
    )]
    async fn save_view(&self, user_credentials_view: UserCredentialsView)-> Result<(), UserViewRepositoryError > {
        sqlx::query("INSERT INTO user_credentials_view (user_email, user_hash, uuid)
                  VALUES ($1, $2, $3) on conflict(user_email) do nothing")
            .bind(user_credentials_view.email)
            .bind(user_credentials_view.hashed_credentials)
            .bind(user_credentials_view.uuid)
            .execute(self.pool)
            .await?
        ;
        Ok(())
    }

    async fn retrieve_user_credentials_view(&self, email: &UserEmail) -> Result<Option<UserCredentialsView>, UserViewRepositoryError> {
        let view = sqlx::query("SELECT FROM user_credentials_view where
                  user_email = $1")
            .bind(email.value())
            .fetch_optional(self.pool)
            .await?
            .map(|row: PgRow| {
                UserCredentialsView{
                    uuid: row.get("uuid"),
                    email: row.get("user_email"),
                    hashed_credentials: row.get("user_hash")
                }
            });
        Ok(view)
    }
}

impl From<sqlx::Error> for UserViewRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(
            "SQLX error for UserViewRepositoryError: {:?}",
            err.as_database_error().unwrap().to_string()
        );
        UserViewRepositoryError::DatabaseConnectionError(err.as_database_error().unwrap().to_string())
    }
}
