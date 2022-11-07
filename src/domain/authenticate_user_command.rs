use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserCommand {
    pub id: Uuid,
    pub email: String,
    pub password_attempt: String,
}
impl AuthenticateUserCommand {
    pub fn new(id: Uuid, email: String, password_attempt: String) -> Self {
        Self {
            id,
            email,
            password_attempt,
        }
    }
}