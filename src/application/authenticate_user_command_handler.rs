use crate::domain::{
    EventStoreInterface, AuthenticateUserCommand, User, UserDomainError, UserEventStoreRepository,
    UserId,
};

#[tracing::instrument(
name = "Authenticate User Command Handler",
skip(user_event_store_repository)
)]
pub async fn authenticate_user_command_handler(
    user_event_store_repository: &UserEventStoreRepository<impl EventStoreInterface<User>>,
    command: AuthenticateUserCommand,
) -> Result<(), UserDomainError> {
    let user_id = UserId::new(command.id);
    let user = user_event_store_repository.load(user_id).await?;
    let events = user.authenticate_user(command)?;
    user_event_store_repository
        .save_events(user_id, events)
        .await

    // To generate or not generate the JWT, well it's not a domain event is it? It;s something
    // my application does, but not something that happened in the domain, because teh domain
    // event was the successful authentication.


}
