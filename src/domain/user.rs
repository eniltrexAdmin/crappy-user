use crate::domain::*;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: UserId, // init default state has not even Id.
    email: UserEmail,
    password: UserPassword,
    is_registered: bool,
}

impl User {
    pub fn new(id: UserId, email: UserEmail, password: UserPassword) -> Self {
        User {
            id,
            email,
            password,
            is_registered: false,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    // all of these as_ref are a little bit fishy, only used for tests
    // and probable what the compiler wanted me to do was to
    // impl AsRef<T> for UserEmail and so on.
    pub fn email_as_ref(&self) -> &UserEmail {
        &self.email
    }

    pub fn password_as_ref(&self) -> &UserPassword {
        &self.password
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    // all domain actions should assume an "init" state. Therefore all have &self.
    // since I decided commands are first class class domain citizens, I am passing it here.
    pub fn register_user(
        &self,
        register_user_command: RegisterUserCommand,
    ) -> Result<Vec<UserDomainEvent>, UserDomainError> {
        if self.is_registered {
            return Err(UserDomainError::UserAlreadyRegistered(
                self.email.value(),
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
            occurred_at: Utc::now()
        };
        Ok(vec![UserDomainEvent::RegisteredUser(user_registered_event)])
    }

    pub fn authenticate_user(
        &self,
        authenticate_user_command: AuthenticateUserCommand,
    )-> Result<Vec<UserDomainEvent>, UserDomainError> {
        if authenticate_user_command.id != *self.id.value() {
            return Err(UserDomainError::CommandNotApplicableToThisUser);
        }
        let user_id = UserId::new(authenticate_user_command.id);
        let result = self.password_as_ref().verify_password(&authenticate_user_command.hashed_password);
        return match result {
            Ok(_) => {
                let event = UserSuccessfullyAuthenticated{
                    id: *user_id.value(),
                    occurred_at: Utc::now()
                };
               Ok(vec![UserDomainEvent::UserAuthenticated(event)])
            },
            Err(UserDomainError::IncorrectPassword) => {
                let event = UserAuthenticationFailed{
                    id: *user_id.value(),
                    occurred_at: Utc::now()
                };
                Ok(vec![UserDomainEvent::UserAuthenticationFailed(event)])
            },
            Err(error) => {
                return Err(error);
            }
        }
    }

    // private
    fn apply_user_registered_event(&mut self, user_registered_event: UserRegisteredDomainEvent) {
        self.id = UserId::new(user_registered_event.id);
        self.is_registered = true;
        self.email =
            UserEmail::new(user_registered_event.email.as_str())
                .unwrap_or_else(|result| panic!("{}", result))
        ;

        self.password = UserPassword::from_storage(
            user_registered_event.password_hash,
            user_registered_event.salt,
        );
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(
            UserId::default(),
            UserEmail::new("init_user@gmail.com").unwrap(),
            UserPassword::new("defaultPassword").unwrap()
        )
    }
}

#[async_trait]
impl EventSourcedAggregate for User {
    type Event = UserDomainEvent;
    type Error = UserDomainError;

    fn aggregate_type() -> String {
        "user".to_string()
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            UserDomainEvent::RegisteredUser(event) => {
                self.apply_user_registered_event(event);
            }
            _ => {}
        }
    }
}
