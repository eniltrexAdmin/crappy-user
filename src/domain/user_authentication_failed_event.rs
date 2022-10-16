use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub const USER_REGISTER_EVENT_TYPE: &str = "UserAuthenticationFailed";
pub const USER_REGISTER_EVENT_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAuthenticationFailed {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>
}
impl UserAuthenticationFailed {
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
