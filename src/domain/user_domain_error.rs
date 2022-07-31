use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum UserDomainError {
    InvalidUuidUserId,
    InvalidUserEmail(String),
    CouldNotGeneratePassword(String),
    ProblemRetrievingPassword(String),
    CommandNotYetImplemented(String)
}
impl std::error::Error for UserDomainError {}
impl Display for UserDomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDomainError::InvalidUuidUserId => write!(f,"Invalid Uuid for User Id"),
            UserDomainError::InvalidUserEmail(error) => write!(f,"InvalidUserEmail: {}", error),
            UserDomainError::CouldNotGeneratePassword(error) => write!(f,"Could Not Generate Password: {}", error),
            UserDomainError::ProblemRetrievingPassword(error) => write!(f,"Could not retrieve password: {}", error),
            UserDomainError::CommandNotYetImplemented(command) => write!(f, "Command {} is not implemented yet.", command)
        }
    }
}


// TODO think if I can either remove this one or the user_domain_error on to avoid having
// too many errors.
#[derive(Debug)]
pub struct UserRegistrationError(String);
impl Display for UserRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}

impl std::error::Error for UserRegistrationError {}

impl From<&str> for UserRegistrationError {
    fn from(message: &str) -> Self {
        UserRegistrationError(message.to_string())
    }
}
