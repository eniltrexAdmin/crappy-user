use sqlx::PgPool;
use crate::domain::{UserCredentialsView, UserViewRepositoryError, UserViewRepositoryInterface};
use async_trait::async_trait;

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
        sqlx::query("INSERT INTO user_credentials_view (user_email, user_hash)
                  VALUES ($1, $2) on conflict(user_email) do nothing")
            .bind(user_credentials_view.email)
            .bind(user_credentials_view.hashed_credentials)
            .execute(self.pool)
            .await?
        ;
        Ok(())
    }
}

impl From<sqlx::Error> for UserViewRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(
            "SQLX error: {:?}",
            err.as_database_error().unwrap().to_string()
        );
        UserViewRepositoryError::DatabaseConnectionError(err.as_database_error().unwrap().to_string())
    }
}
