use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use uuid::Uuid;
use crate::application::authenticate_user_command_handler;
use crate::domain::{AuthenticateUserCommand, EventStoreInterface, User, UserDomainError, UserDomainEvent, UserEmail, UserEventStoreRepository, UserViewRepositoryInterface};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CrappyUserClaims {
    sub: String,
    user_id: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserApplicationRequest {
    email: String,
    password_attempt: String,
}

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
    let view_user = view_repository.retrieve_user_credentials_view(&user_email).await?;

    let authenticate_user_command = AuthenticateUserCommand{
        id: view_user.uuid,
        email: view_user.email.clone(),
        hashed_password:  authenticate_user_request.password_attempt
    };

    let authenticate_user_result = authenticate_user_command_handler(
        &user_event_store_repository,
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
