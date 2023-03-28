use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use secrecy::{SecretString};
use uuid::Uuid;
use crate::application::{authenticate_user_command_handler, AuthenticateUserCommand};
use crate::domain::{EventStoreInterface, User, UserDomainError, UserDomainEvent, UserEmail, UserViewRepositoryInterface, UserEventStoreRepository};
use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct CrappyUserClaims {
    sub: String,
    user_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct AuthenticateUserApplicationRequest {
    email: String,
    password_attempt: SecretString,
}

impl Serialize for AuthenticateUserApplicationRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.email)
    }
}
// Hey did I see something weird here?
// I wrote a lot on previous commits here, and today I realized what the main difference
// between Garrofolo and me here: I emit domain events.
// If I emit domain events, it's not a query, it's a command!, so I should not
// use the read repository, but the write, and only the user event store reposirtory.
// So here it's the "fishy" way, I still kind of like it to break the rules, once
// I know which rules I am breaking.
// That's why I am calling the command handler...and the event store, it is behaving
// like a controller this whole 'application'
// And the only reasong I hve the view repository, which the initial idea was to do the
// actual match of credentials, is now to find the user beforehand
// I might probably remove the whole 'application' or well, leave it as a testament(?)
// like we can have "satellites' and that's how you plug them in in the system.
#[tracing::instrument(
name = "Authenticate application",
skip(view_repository, user_event_store_repository)
)]
pub async fn crappy_authenticate_user(
    authenticate_user_request: AuthenticateUserApplicationRequest,
    view_repository: &impl UserViewRepositoryInterface,
    user_event_store_repository: &UserEventStoreRepository<impl EventStoreInterface<User>>
) -> Result<String, UserDomainError>  {
    let user_email = UserEmail::new(authenticate_user_request.email.as_str())?;
    let user_found = view_repository.retrieve_user_credentials_view(&user_email)
        .await?;

    if user_found.is_none() {
        return Err(UserDomainError::UserNotFound("Probably the View has not been generated yet.".to_string()));
    }
    let view_user = user_found.unwrap();

    let authenticate_user_command = AuthenticateUserCommand{
        id: view_user.uuid,
        email: view_user.email.clone(),
        password_attempt:  authenticate_user_request.password_attempt
    };

    let authenticate_user_result = authenticate_user_command_handler(
        user_event_store_repository,
        authenticate_user_command
    ).await?;

    let user_claims = CrappyUserClaims {
        sub: view_user.email,
        user_id:  view_user.uuid
    };

    return match authenticate_user_result {
        UserDomainEvent::UserAuthenticated(_) => {
            let encoding_key = &EncodingKey::from_rsa_pem(include_bytes!("private.pem"))
                .map_err(|e| {
                    UserDomainError::CouldNotGeneratePassword(format!("Could not generate JWT: {}",e.to_string()))
                })?;
            let token = encode(&Header::new(Algorithm::RS256), &user_claims, encoding_key)
                .map_err(|e| {
                    UserDomainError::CouldNotGeneratePassword(format!("Could not generate JWT: {}",e.to_string()))
                })?;
            Ok(token)
        },
        UserDomainEvent::UserAuthenticationFailed(_) => Err(UserDomainError::IncorrectPassword),
        _ => Err(UserDomainError::UnexpectedDomainEvent),
    }
}
