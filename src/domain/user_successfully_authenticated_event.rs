use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::DomainEvent;
use uuid::Uuid;

pub const USER_REGISTER_EVENT_TYPE: &str = "UserAuthenticated";
pub const USER_REGISTER_EVENT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSuccessfullyAuthenticated {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>
}
impl UserSuccessfullyAuthenticated {
    pub fn event_type(&self) -> String {
        String::from(USER_REGISTER_EVENT_TYPE)
    }

    pub fn event_version(&self) -> String {
        String::from(USER_REGISTER_EVENT_VERSION)
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
