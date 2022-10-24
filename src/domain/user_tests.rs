#[cfg(test)]
mod tests {
    use actix_web::body::MessageBody;
    use crate::domain::*;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn new_user_init() {
        let id = UserId::default();
        let email = UserEmail::new("francesc.travesa@mymail.com").unwrap();
        let password_hash = "password_hash".to_string();
        let user_password = UserPassword::new(&password_hash).unwrap();
        let user = User::new(id, email.clone(), user_password.clone());
        assert_eq!(user.id(), id);
        assert_eq!(false, user.is_registered());
        assert_eq!(&email, user.email_as_ref());
        assert_eq!(&user_password, user.password_as_ref());
    }

    #[test]
    fn apply_register_event() {
        let id = UserId::default();
        let email = UserEmail::new("francesc.travesa@mymail.com").unwrap();
        let password_hash = "password_hash".to_string();
        let user_password = UserPassword::new(&password_hash).unwrap();
        let salt = "salt".to_string();

        let register_domain_event = UserDomainEvent::RegisteredUser(UserRegisteredDomainEvent {
            id: *id.value(),
            email: email.value(),
            password_hash: password_hash.clone(),
            salt: salt.clone(),
            occurred_at: Utc::now()
        });

        let mut user = User::new(id, email.clone(), user_password);

        user.apply(register_domain_event);

        assert_eq!(user.id(), id);
        assert_eq!(true, user.is_registered());
        assert_eq!(&email, user.email_as_ref());
        assert_eq!(
            password_hash,
            user.password_as_ref().hash_string
        );
        assert_eq!(salt, user.password_as_ref().salt);
    }

    #[test]
    fn register_user_command() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();

        let command = RegisterUserCommand::new(id, email.clone(), "mySecretPassword".to_string());
        // because I am not saving commands, but if I were, the above password should be already hashed.

        let user = User::default();
        let result = user.register_user(&command);
        let events = result.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();

        match event {
            UserDomainEvent::RegisteredUser(user_registered_event) => {
                assert_eq!(user_registered_event.id, id);
                assert_eq!(user_registered_event.email, email);
                assert_eq!(user_registered_event.password_hash.is_empty(), false);
                assert_eq!(user_registered_event.salt.is_empty(), false);
            },
            _=>{}
        }
    }

    #[test]
    fn test_register_user_idempotency() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let previous = UserDomainEvent::RegisteredUser(UserRegisteredDomainEvent {
            id: id.clone(),
            email: email.clone(),
            password_hash: "".to_string(),
            salt: "".to_string(),
            occurred_at: Utc::now()
        });

        let mut user = User::default();
        user.apply(previous);

        let command = RegisterUserCommand::new(id, email.clone(), "mySecretPassword".to_string());

        let result = user.register_user(&command);

        assert_eq!(result, Err(UserDomainError::UserAlreadyRegistered(email)))
    }

    #[test]
    fn authenticate_user_command() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();

        let command = AuthenticateUserCommand::new(id, email.clone(), "hashedPassword".to_string());
        // because I am not saving commands, but if I were, the above password should be already hashed.

        let user = User::default();
        let result = user.au(&command);
        let events = result.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();

        match event {
            UserDomainEvent::RegisteredUser(user_registered_event) => {
                assert_eq!(user_registered_event.id, id);
                assert_eq!(user_registered_event.email, email);
                assert_eq!(user_registered_event.password_hash.is_empty(), false);
                assert_eq!(user_registered_event.salt.is_empty(), false);
            },
            _=>{}
        }
    }
}
