use serde::{Deserialize, Serialize};

use crate::domain::DomainEvent;
use uuid::Uuid;

pub const USER_REGISTER_EVENT_TYPE: &str = "UserRegistered";
pub const USER_REGISTER_EVENT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserRegisteredDomainEvent {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
}
impl DomainEvent for UserRegisteredDomainEvent {
    fn event_type(&self) -> String {
        String::from(USER_REGISTER_EVENT_TYPE)
    }

    fn event_version(&self) -> String {
        String::from(USER_REGISTER_EVENT_VERSION)
    }
}
