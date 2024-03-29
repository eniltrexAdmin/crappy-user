#[cfg(test)]
pub(crate) mod tests {
    use crate::domain::*;
    use uuid::Uuid;
    use chrono::{SubsecRound, Utc};

    pub fn default_user() -> User {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com";
        let password_hash = "password_hash";
        let user = simulate_fetch_user(
            id,
            email.clone(),
            password_hash
        );
        return user;
    }

    pub fn simulate_fetch_user(id: Uuid, email: &str, password_hash: &str) -> User {
        let user_id = UserId::new(id);
        let email = UserEmail::new(email).unwrap();
        let password_hash = password_hash.to_string();
        let user_password = UserPassword::new(&password_hash).unwrap();

        let register_domain_event = UserDomainEvent::RegisteredUser(UserRegisteredDomainEvent {
            id: *user_id.value(),
            email: email.value(),
            password_hash: user_password.hash_string,
            salt: user_password.salt,
            occurred_at: Utc::now()
        });

        let mut user = User::default();
        user.apply(register_domain_event);
        user
    }

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
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let password_hash = "$argon2id$v=19$m=4096,t=3,p=1$HSe675tHxpc4wepbYnOk9Q$fEhOO9euXWL0F2i1bIia8PffBWbhbmX29CzlwfYjno4".to_string();
        let salt = "salt".to_string();
        let register_domain_event = UserDomainEvent::RegisteredUser(UserRegisteredDomainEvent {
            id,
            email: email.clone(),
            password_hash: password_hash.clone(),
            salt: salt.clone(),
            occurred_at: Utc::now()
        });

        let mut user = User::default();
        user.apply(register_domain_event);

        assert_eq!(user.id().value(), &id);
        assert_eq!(true, user.is_registered());
        assert_eq!(email, user.email_as_ref().value());
        assert_eq!(
            password_hash,
            user.password_as_ref().hash_string
        );
        assert_eq!(
            salt,
            user.password_as_ref().salt
        );
    }

    #[test]
    fn apply_successful_login_event(){
        let mut user = default_user();
        let pre_id = user.id().clone();
        let pre_email = user.email_as_ref().clone();
        let pre_password = user.password_as_ref().clone();
        let successful_login_event = UserDomainEvent::UserAuthenticated(UserSuccessfullyAuthenticated{
            id: *user.id().value(),
            occurred_at: Utc::now()
        });
        user.apply(successful_login_event);
        assert_eq!(pre_id, user.id());
        assert_eq!(&pre_email, user.email_as_ref());
        assert_eq!(&pre_password, user.password_as_ref());
    }

    #[test]
    fn apply_unsuccessful_login_event(){
        let mut user = default_user();
        let pre_id = user.id().clone();
        let pre_email = user.email_as_ref().clone();
        let pre_password = user.password_as_ref().clone();
        let successful_login_event = UserDomainEvent::UserAuthenticationFailed(UserAuthenticationFailed{
            id: *user.id().value(),
            occurred_at: Utc::now()
        });
        user.apply(successful_login_event);
        assert_eq!(pre_id, user.id());
        assert_eq!(&pre_email, user.email_as_ref());
        assert_eq!(&pre_password, user.password_as_ref());
    }

    #[test]
    fn register_user() {
        let id = UserId::new(Uuid::new_v4());
        let email = UserEmail::new("francesc.travesa@mymail.com".to_string().as_ref()).unwrap();
        let password = UserPassword::new( "my_password".to_string().as_ref()).unwrap();

        let mut user = User::default();
        user.register_user(id, email.clone(), password).unwrap();
        let events = user.recorded_events();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();

        match event {
            UserDomainEvent::RegisteredUser(user_registered_event) => {
                assert_eq!(user_registered_event.id, *id.value());
                assert_eq!(user_registered_event.email, email.value());
                assert_eq!(user_registered_event.password_hash.is_empty(), false);
                assert_eq!(user_registered_event.salt.is_empty(), false);
                assert_eq!(
                    Utc::now().round_subsecs(2),
                    user_registered_event.occurred_at().round_subsecs(2)
                );
            },
            _=>{}
        }

        assert_eq!(user.id(), id);
        assert_eq!(true, user.is_registered());
        assert_eq!(email, *user.email_as_ref());
    }

    #[test]
    fn test_register_user_idempotency() {
        let id = UserId::new(Uuid::new_v4());
        let email = UserEmail::new("francesc.travesa@mymail.com".to_string().as_ref()).unwrap();
        let password = UserPassword::new( "my_password".to_string().as_ref()).unwrap();
        let mut user = simulate_fetch_user(
            *id.value(),
            email.clone().value().as_ref(),
            password.hash_string.as_str()
        );
        let result = user.register_user(id, email.clone(), password);
        assert_eq!(result, Err(UserDomainError::UserAlreadyRegistered(email.value())))
    }

    #[test]
    fn authenticate_user() {
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let password_hash = "password_hash".to_string();
        let user = simulate_fetch_user(id, &email, &password_hash);

        let result = user.authenticate_user(&password_hash);
        let events = result.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();

        match event {
            UserDomainEvent::UserAuthenticated(user_authenticated_event) => {
                assert_eq!(user_authenticated_event.id, id);
                assert_eq!(
                    Utc::now().round_subsecs(3),
                    user_authenticated_event.occurred_at().clone().round_subsecs(3)
                );
            },
            wrong_domain_event=> {
                assert_eq!(
                    true,
                    false,
                    "event generated was not of type UserAuthenticated but of type {}",
                    wrong_domain_event.event_type()
                );
            }
        }
        // Now failed.
        let result = user.authenticate_user("wrong_password".to_string().as_ref());
        let events = result.unwrap();
        assert_eq!(1, events.len());
        let event = events.get(0).unwrap();
        match event {
            UserDomainEvent::UserAuthenticationFailed(user_authenticated_event) => {
                assert_eq!(user_authenticated_event.id, id);
            },
            wrong_domain_event=>{
                assert_eq!(
                    true,
                    false,
                    "event generated was not of type UserAuthenticated but of type {}",
                    wrong_domain_event.event_type()
                );
            }
        }
    }
}
