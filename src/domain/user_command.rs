use serde::{Serialize, Deserialize};
use crate::domain::RegisterUserCommand;

#[derive(Serialize, Deserialize)]
pub enum UserCommand{
    RegisterUser(RegisterUserCommand)
}

impl From<UserCommand> for String {
    fn from(user_command: UserCommand) -> Self {
        return match user_command {
            UserCommand::RegisterUser(_) => {
                "RegisterUserCommand".to_string()
            }
        }
    }
}
