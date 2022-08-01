use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::*;
use async_trait::async_trait;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<UserId>, // init state has not even Id.
    email: Option<UserEmail>,
    password: Option<UserPassword>,
    is_registered: bool
}


impl User {
    pub fn new() -> Self {
        User {
            id: None,
            email: None,
            password:None,
            is_registered: false
        }
    }

    pub fn id(&self) -> Option<UserId> {
        self.id
    }

    pub fn email(self) -> Option<UserEmail> {
        self.email
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    // all domain actions should assume an "init" state. Therefore all have &self.
    pub fn register_user(&self, id:Uuid, email:&str, password: &str) -> Result<Vec<UserEvent>, Self::Error> {
        if self.is_registered {
            return Err(UserDomainError::UserAlreadyRegistered(self.email.as_ref().unwrap().value()));
        }

        let user_id = UserId::new(id);
        let user_email = UserEmail::new(email)?;
        let user_password = UserPassword::new(password)?;

        let user_registered_event = UserRegisteredDomainEvent{
            id: *user_id.value(),
            email: user_email.value(),
            password_hash: user_password.hash_string,
            salt: user_password.salt
        };
        // should I return that or the user?
        Ok(vec![UserEvent::RegisteredUser(user_registered_event)])
    }

    pub fn apply_user_registered_event(&mut self, user_registered_event: UserRegisteredDomainEvent) {
        self.is_registered = true;
        self.email = Some(UserEmail::new(user_registered_event.email.as_str()).unwrap_or_else(|result| {
            panic!("{}", result)
        }));

        self.password = Some(UserPassword::from_storage(
            user_registered_event.password_hash,
             user_registered_event.salt
        ));
    }
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}

#[async_trait]
impl Aggregate for User {
    type Command = UserCommand;
    type Event = UserEvent;
    type Error = UserDomainError;
    type Services = ();

    fn aggregate_type() -> String {
        "user".to_string()
    }

    // also what about checking the email already exists? I need to query the view first.
    async fn handle(&self, command: Self::Command, _service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
       match command{
           UserCommand::RegisterUser(register_user_command) => {
               if self.is_registered {
                   return Err(UserDomainError::UserAlreadyRegistered(self.email.as_ref().unwrap().value()));
               }
               // I need to validate here the stuff before publishing the domain event.
               let user_email = UserEmail::new(register_user_command.email.as_str())?;
               let user_password = UserPassword::new(register_user_command.password.as_str())?;

               let user_registered_event = UserRegisteredDomainEvent{
                   id: register_user_command.id,
                   email: user_email.value(),
                   password_hash: user_password.hash_string,
                   salt: user_password.salt
               };
               Ok(vec![UserEvent::RegisteredUser(user_registered_event)])
           },
           not_implemented_command => Err(UserDomainError::CommandNotYetImplemented(not_implemented_command.into()))
       }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            UserEvent::RegisteredUser(event) => {
                self.apply_user_registered_event(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_ok;
    use uuid::Uuid;
    use super::*;
    use cqrs_es::test::TestFramework;
    type UserTestFramework = TestFramework<User>;

    #[test]
    fn new_user() {

        let user = User::new();
        assert_eq!(user.id(), None);
        assert_eq!(false, user.is_registered());
        assert_eq!(None, user.email());
    }

    #[tokio::test]
    async fn test_register_user() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();

        let command = UserCommand::RegisterUser(RegisterUserCommand::new(
            id,
            email.clone(),
             "mySecretPassword".to_string()
        ));

        let user = User::default();
        let result = user.handle(command,&()).await;
        let events = result.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();

        match event {
            UserEvent::RegisteredUser(user_registered_event) => {
                assert_eq!(user_registered_event.id, id);
                assert_eq!(user_registered_event.email, email);
                assert_eq!(user_registered_event.password_hash.is_empty(), false);
                assert_eq!(user_registered_event.salt.is_empty(), false);
            }
        }
    }

    #[test]
    fn test_register_user_idempotency() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let previous = UserEvent::RegisteredUser(UserRegisteredDomainEvent{
            id: id.clone(),
            email:  email.clone(),
            password_hash: "".to_string(),
            salt: "".to_string()
        });
        let command = UserCommand::RegisterUser(RegisterUserCommand::new(
            id,
            email.clone(),
            "mySecretPassword".to_string()
        ));
        UserTestFramework::with(())
            .given(vec![previous])
            .when(command)
            .then_expect_error(UserDomainError::UserAlreadyRegistered("francesc.travesa@mymail.com".to_string()))
    }

}


