use crate::domain::{EventStoreInterface, User, UserCredentialsView, UserViewRepositoryError, UserViewRepositoryInterface};
use crate::domain::UserEvent::RegisteredUser;

#[tracing::instrument(
name = "User Credentials aggregator",
skip(event_store, view_repository)
)]
pub async fn user_credentials_aggregator(
    event_store:  impl EventStoreInterface<User>,
    view_repository: impl UserViewRepositoryInterface,
    last_read_event: i64
) -> Result<(), UserViewRepositoryError>  {
    let events = event_store.load_all_events(last_read_event).await?;
    for event in events {
        match event.payload {
            RegisteredUser(user_registered_domain_event) => {
                let user_credentials_view: UserCredentialsView = user_registered_domain_event.into();
                view_repository.save_view(user_credentials_view).await?;
            }
        }
    }
    Ok(())
}
