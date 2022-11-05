use uuid::Uuid;
use crate::domain::{EventStoreInterface, AuthenticateUserCommand, User, UserDomainError, UserEventStoreRepository, UserId, UserDomainEvent, EventStoreError};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CrappyUserClaims {
    sub: String,
    user_id: Uuid
}

#[tracing::instrument(
name = "Authenticate User Command Handler",
skip(user_event_store_repository)
)]
pub async fn authenticate_user_command_handler(
    user_event_store_repository: &UserEventStoreRepository<impl EventStoreInterface<User>>,
    command: AuthenticateUserCommand,
) -> Result<String, UserDomainError> {
    let user_id = UserId::new(command.id);
    let user = user_event_store_repository.load(user_id).await?;
    let events = user.authenticate_user(command)?;
    let authentication_result = events.first().unwrap().clone();
    user_event_store_repository
        .save_events(user_id, events)
        .await?;

    // To generate or not generate the JWT, well it's not a domain event is it? It's something
    // my application does, but not something that happened in the domain, because the domain
    // event was the successful authentication.

    let user_claims = CrappyUserClaims {
        sub: user.email_as_ref().value(),
        user_id:  user.id().value().to_owned()
    };

    return match authentication_result {
        UserDomainEvent::UserAuthenticated(_) => {
            // TODO get this value from the config!!! not hardcoded string. Take into account that
            // this is needed to make the build, which I kind of like it.
            let encoding_key = &EncodingKey::from_rsa_pem(include_bytes!("../config/private.pem"))
                .map_err(|e| {
                    UserDomainError::CouldNotGeneratePassword("Could not generate JWT".to_string())
                })?;
            let token = encode(&Header::new(Algorithm::RS256), &user_claims, encoding_key)
                .map_err(|e| {
                    UserDomainError::CouldNotGeneratePassword("Could not generate JWT".to_string())
                })?;
            Ok(token)
        },
        UserDomainEvent::UserAuthenticationFailed(_) => Err(UserDomainError::IncorrectPassword),
        _ => Err(UserDomainError::UnexpectedDomainEvent),
    }
}

// TODO add test for this comand and the register too, mock the repository. Should be OK
