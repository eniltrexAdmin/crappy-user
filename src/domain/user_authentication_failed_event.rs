use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use crate::domain::DomainEvent;

pub const USER_REGISTER_EVENT_TYPE: &str = "UserAuthenticationFailed";
pub const USER_REGISTER_EVENT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAuthenticationFailed {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>
}
impl DomainEvent for UserAuthenticationFailed {
    fn event_type(&self) -> String {
        String::from(USER_REGISTER_EVENT_TYPE)
    }

    fn event_version(&self) -> String {
        String::from(USER_REGISTER_EVENT_VERSION)
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
