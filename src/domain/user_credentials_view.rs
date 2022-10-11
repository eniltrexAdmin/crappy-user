use crate::domain::{UserRegisteredDomainEvent};

#[derive(Debug)]
pub struct UserCredentialsView{
    pub email: String,
    pub hashed_credentials: String,
}

impl From<UserRegisteredDomainEvent> for UserCredentialsView {
    fn from(event: UserRegisteredDomainEvent) -> Self {
        Self{
            email: event.email,
            hashed_credentials: event.password_hash
        }
    }
}

