use crate::domain::UserRegistrationError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserCommand {
    email: String,
    password_hash: String
}

pub async fn register_user_command_handler() -> Result<(), UserRegistrationError> {
    Ok(())
}

