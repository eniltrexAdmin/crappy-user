use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use cqrs_es::DomainEvent;

const USER_REGISTER_EVENT_TYPE: &str = "UserRegistered";
const USER_REGISTER_EVENT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserRegisteredDomainEvent {
    email: String,
    password_hash: String,
    salt: String
}
impl DomainEvent for UserRegisteredDomainEvent {
    fn event_type(&self) -> String {
        String::from(USER_REGISTER_EVENT_TYPE)
    }

    fn event_version(&self) -> String {
        String::from(USER_REGISTER_EVENT_VERSION)
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