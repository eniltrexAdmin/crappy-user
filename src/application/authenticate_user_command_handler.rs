use crate::domain::{EventStoreInterface, AuthenticateUserCommand, User, UserDomainError, UserEventStoreRepository, UserId, UserDomainEvent};

#[tracing::instrument(
name = "Authenticate User Command Handler",
skip(user_event_store_repository)
)]
pub async fn authenticate_user_command_handler(
    user_event_store_repository: &UserEventStoreRepository<impl EventStoreInterface<User>>,
    command: AuthenticateUserCommand,
) -> Result<UserDomainEvent, UserDomainError> {
    let user_id = UserId::new(command.id);
    let user = user_event_store_repository.load(user_id).await?;
    let events = user.authenticate_user(command)?;
    let authentication_result = events.first().unwrap().clone();
    user_event_store_repository
        .save_events(user_id, events)
        .await?;
    return Ok(authentication_result);
}

// TODO add test for this command and the register too, mock the repository. Should be OK
