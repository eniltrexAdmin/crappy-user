use crate::domain::{EventSourcedAggregate, RegisterUserCommand, UserDomainError, UserId, UserRepository};

#[tracing::instrument(
    name = "Register User Command Handler",
    skip(user_event_store_repository)
)]
pub async fn register_user_command_handler(
    user_event_store_repository: &impl UserRepository,
    command: RegisterUserCommand,
) -> Result<(), UserDomainError> {
    let user_id = UserId::new(command.id);
    let mut user = user_event_store_repository.load(user_id).await?;
    user.register_user(command)?;
    user_event_store_repository
        .save_events(user_id, user.recorded_events())
        .await
}
