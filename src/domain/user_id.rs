use uuid::Uuid;
use crate::domain::UserDomainError;
use serde::{Deserialize, Serialize};

// Serialize and deserialize because User has it.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn from_string(value: &str) ->  Result<Self, UserDomainError> {
        let uuid = match Uuid::parse_str(value) {
            Err(_) => return Err(UserDomainError::InvalidUuidUserId),
            Ok(uuid) => uuid
        };
        Ok(Self(uuid))
    }
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn user_id_value() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::new(uuid);
        assert_eq!(&uuid, user_id.value())
    }

    #[test]
    fn user_id_creation() {
        let string_uuid = Uuid::new_v4().to_string();
        let user_id = UserId::from_string(string_uuid.as_str());
        assert_ok!(user_id);
    }
    #[test]
    fn user_id_creation_invalid() {
        let user_id = UserId::from_string("");
        assert_err!(user_id);
        let user_id = UserId::from_string("");
        assert!(matches!(user_id, Err(UserDomainError::InvalidUuidUserId)));
    }
}
