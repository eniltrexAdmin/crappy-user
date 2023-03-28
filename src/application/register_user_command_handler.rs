use crate::application::RegisterUserCommand;
use crate::domain::{EventSourcedAggregate, UserDomainError, UserEmail, UserId, UserPassword, UserRepository};

#[tracing::instrument(
    name = "Register User Command Handler",
    skip(user_event_store_repository)
)]
pub async fn register_user_command_handler(
    user_event_store_repository: &impl UserRepository,
    command: RegisterUserCommand,
) -> Result<(), UserDomainError> {
    // either form is valid, construct value objects here or pass simple and construct inside user.
    let user_id = UserId::new(command.id);
    let user_email = UserEmail::new(command.email.as_str())?;
    let user_password = UserPassword::new(command.password.as_str())?;

    let mut user = user_event_store_repository.load(user_id).await?;
    user.register_user(user_id, user_email, user_password)?;
    user_event_store_repository
        .save_events(user_id, user.recorded_events())
        .await
}
