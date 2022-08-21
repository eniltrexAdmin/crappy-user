use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, PartialEq)]
pub enum UserDomainError {
    InvalidUuidUserId,
    InvalidUserEmail(String),
    CouldNotGeneratePassword(String),
    ProblemRetrievingPassword(String),
    UserAlreadyRegistered(String),
    CommandNotYetImplemented(String),
    CouldNotLoadUserEvents(String),
    CouldNotSaveUserEvents(String),
}
impl std::error::Error for UserDomainError {}
impl Display for UserDomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDomainError::InvalidUuidUserId => write!(f, "Invalid Uuid for User Id"),
            UserDomainError::InvalidUserEmail(error) => write!(f, "InvalidUserEmail: {}", error),
            UserDomainError::CouldNotGeneratePassword(error) => {
                write!(f, "Could Not Generate Password: {}", error)
            }
            UserDomainError::ProblemRetrievingPassword(error) => {
                write!(f, "Could not retrieve password: {}", error)
            }
            UserDomainError::CommandNotYetImplemented(command) => {
                write!(f, "Command {} is not implemented yet.", command)
            }
            UserDomainError::UserAlreadyRegistered(error) => {
                write!(f, "User {} is already registered.", error)
            }
            UserDomainError::CouldNotLoadUserEvents(error) => {
                write!(f, "Problem loading user: {}", error)
            },
            UserDomainError::CouldNotSaveUserEvents(error) => {
                write!(f, "Problem saving user events: {}", error)
            }
        }
    }
}
