use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserCommand {
    pub id: Uuid,
    pub email: String,
    pub password: String
}
impl RegisterUserCommand {
    pub fn new(id: Uuid, email: String, password: String) -> Self {
        Self {
            id,
            email,
            password
        }
    }
}
