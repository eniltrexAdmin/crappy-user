use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum UserDomainError {
    InvalidUuidUserId,
    InvalidUserEmail(String),
    CouldNotGeneratePassword(String),
    ProblemRetrievingPassword(String)
}
impl std::error::Error for UserDomainError {}
impl Display for UserDomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDomainError::InvalidUuidUserId => write!(f,"Invalid Uuid for User Id"),
            UserDomainError::InvalidUserEmail(error) => write!(f,"InvalidUserEmail: {}", error),
            UserDomainError::CouldNotGeneratePassword(error) => write!(f,"Could Not Generate Password: {}", error),
            UserDomainError::ProblemRetrievingPassword(error) => write!(f,"Could not retrieve password: {}", error),
        }
    }
}

