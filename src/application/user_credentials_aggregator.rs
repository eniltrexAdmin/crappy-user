use crate::domain::{EventStoreInterface, User, UserEventStoreRepository};

#[tracing::instrument(
name = "User Credentials aggregator",
skip(user_event_store_repository)
)]
pub async fn register_user_command_handler(
    event_store:  impl EventStoreInterface<User>,
    last_read_event: u64
){
    let events = event_store.load_all_events(last_read_event).await?;

}