use std::error::Error;
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use serde::Serialize;
use crate::domain::{EventStoreError, UserCredentialsView, UserDomainError, UserEmail};

#[async_trait]
pub trait UserViewRepositoryInterface {
    // TODO change that for getting the generic read model. (the two function commented below)
    async fn save_view(&self, user_credentials_view: UserCredentialsView) -> Result<(), UserViewRepositoryError>;
    async fn retrieve_user_credentials_view(&self, email: &UserEmail) -> Result<Option<UserCredentialsView>, UserViewRepositoryError>;

    // async fn save(&self, match_request: MatchRequest) -> Result<(), MatchRequestDomainError>;
    // async fn search_by_criteria(&self, match_request_criteria: MatchRequestCriteria) -> Result<Vec<MatchRequest>, MatchRequestDomainError>;
}


#[derive(Debug, Serialize)]
pub enum UserViewRepositoryError {
    DatabaseConnectionError(String),
}

impl Display for UserViewRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserViewRepositoryError::DatabaseConnectionError(error) => {
                write!(f, "Database problem: {}.", error)
            }
        }
    }
}
impl Error for UserViewRepositoryError {}

impl From<EventStoreError> for UserViewRepositoryError {
    fn from(err: EventStoreError ) -> Self {
        UserViewRepositoryError::DatabaseConnectionError(err.to_string())
    }
}


impl From<UserViewRepositoryError> for UserDomainError {
    fn from(err: UserViewRepositoryError) -> Self {
        return match err {
            UserViewRepositoryError::DatabaseConnectionError(message) => {
                UserDomainError::CouldNotLoadUserView(message)
            }
        };
    }
}
