use crate::domain::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<UserId>, // init state has not even Id.
    email: Option<UserEmail>,
    password: Option<UserPassword>,
    is_registered: bool,
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            email: None,
            password: None,
            is_registered: false,
        }
    }

    pub fn id(&self) -> Option<UserId> {
        self.id
    }

    pub fn email_as_ref(&self) -> &Option<UserEmail> {
        &self.email
    }

    pub fn password_as_ref(&self) -> &Option<UserPassword> {
        &self.password
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    // all domain actions should assume an "init" state. Therefore all have &self.
    // since I decided commands are first class class domain citizens, I am passing it here.
    pub fn register_user(
        &self,
        register_user_command: &RegisterUserCommand,
    ) -> Result<Vec<UserEvent>, UserDomainError> {
        if self.is_registered {
            return Err(UserDomainError::UserAlreadyRegistered(
                self.email.as_ref().unwrap().value(),
            ));
        }

        let user_id = UserId::new(register_user_command.id);
        let user_email = UserEmail::new(register_user_command.email.as_str())?;
        let user_password = UserPassword::new(register_user_command.password.as_str())?;

        let user_registered_event = UserRegisteredDomainEvent {
            id: *user_id.value(),
            email: user_email.value(),
            password_hash: user_password.hash_string,
            salt: user_password.salt,
        };
        // should I return that or the user?
        // copying more form cqrs_es:rust, and going for the events. even we do not need
        // to care that the user has been "modified", it doesn't need to be even mut.
        Ok(vec![UserEvent::RegisteredUser(user_registered_event)])
    }

    // private
    fn apply_user_registered_event(&mut self, user_registered_event: UserRegisteredDomainEvent) {
        self.is_registered = true;
        self.email = Some(
            UserEmail::new(user_registered_event.email.as_str())
                .unwrap_or_else(|result| panic!("{}", result)),
        );

        self.password = Some(UserPassword::from_storage(
            user_registered_event.password_hash,
            user_registered_event.salt,
        ));
    }
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}

#[async_trait]
impl EventSourcedAggregate for User {
    type Event = UserEvent;
    type Error = UserDomainError;

    fn aggregate_type() -> String {
        "user".to_string()
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
    use super::*;
    use uuid::Uuid;

    #[test]
    fn new_user_init() {
        let user = User::new();
        assert_eq!(user.id(), None);
        assert_eq!(false, user.is_registered());
        assert_eq!(&None, user.email_as_ref());
        assert_eq!(&None, user.password_as_ref());

        let user2 = User::default();
        assert_eq!(user2.id(), None);
        assert_eq!(false, user2.is_registered());
        assert_eq!(&None, user2.email_as_ref());
        assert_eq!(&None, user2.password_as_ref());
    }

    #[test]
    fn apply_register_event() {
        let mut user = User::new();
        let email = UserEmail::new("francesc.travesa@mymail.com").unwrap();
        let password_hash = "password_hash".to_string();
        let salt = "salt".to_string();

        let register_domain_event = UserEvent::RegisteredUser(UserRegisteredDomainEvent {
            id: Default::default(),
            email: email.value(),
            password_hash: password_hash.clone(),
            salt: salt.clone(),
        });

        user.apply(register_domain_event);

        assert_eq!(user.id(), None);
        assert_eq!(true, user.is_registered());
        assert_eq!(&Some(email.clone()), user.email_as_ref());
        assert_eq!(
            password_hash,
            user.password_as_ref().as_ref().unwrap().hash_string
        );
        assert_eq!(salt, user.password_as_ref().as_ref().unwrap().salt);
    }

    #[test]
    fn register_user_command() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();

        let command = RegisterUserCommand::new(id, email.clone(), "mySecretPassword".to_string());

        let user = User::default();
        let result = user.register_user(&command);
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
        let previous = UserEvent::RegisteredUser(UserRegisteredDomainEvent {
            id: id.clone(),
            email: email.clone(),
            password_hash: "".to_string(),
            salt: "".to_string(),
        });

        let mut user = User::default();
        user.apply(previous);

        let command = RegisterUserCommand::new(id, email.clone(), "mySecretPassword".to_string());

        let result = user.register_user(&command);

        assert_eq!(result, Err(UserDomainError::UserAlreadyRegistered(email)))
    }
}
