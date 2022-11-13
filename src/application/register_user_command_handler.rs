use crate::domain::{RegisterUserCommand, UserDomainError, UserId, UserRepository};

#[tracing::instrument(
    name = "Register User Command Handler",
    skip(user_event_store_repository)
)]
pub async fn register_user_command_handler(
    user_event_store_repository: &impl UserRepository,
    command: RegisterUserCommand,
) -> Result<(), UserDomainError> {
    let user_id = UserId::new(command.id);
    let user = user_event_store_repository.load(user_id).await?;
    let events = user.register_user(command)?;
    user_event_store_repository
        .save_events(user_id, events)
        .await
}
