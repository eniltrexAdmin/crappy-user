use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserCommand {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
}
impl AuthenticateUserCommand {
    pub fn new(id: Uuid, email: String, hashed_password: String) -> Self {
        Self {
            id,
            email,
            hashed_password,
        }
    }
}
