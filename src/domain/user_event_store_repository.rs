use crate::domain::{DomainEvent, EventEnvelope, EventSourcedAggregate, EventStoreError, EventStoreInterface, User, UserDomainError, UserDomainEvent, UserId};
use async_trait::async_trait;
// use mockall::*;
// that could be even more generic, but I dont want to think about
// a generic UUID to "load", that had to be generic too.
// #[automock]
#[async_trait]
pub trait UserRepository {
     async fn load(&self, user_id: UserId) -> Result<User, UserDomainError>;
     async fn save_events(
        &self,
        user_id: UserId,
        events: Vec<UserDomainEvent>,
    ) -> Result<(), UserDomainError>;
}

pub struct UserEventStoreRepository<ES>
where
    ES: EventStoreInterface<User>,
{
    pub store: ES,
}

#[async_trait]
impl<ES> UserRepository for UserEventStoreRepository<ES>
where
    ES: EventStoreInterface<User>,
{
    async fn load(&self, user_id: UserId) -> Result<User, UserDomainError> {
        let events_to_apply = self.store.load_events(user_id.value()).await.map_err(
            |event_store_error: EventStoreError| {
                UserDomainError::CouldNotLoadUserEvents(event_store_error.to_string())
            },
        )?;
        let mut user = User::default();
        for event in events_to_apply {
            user.apply(event.payload);
        }
        Ok(user)
    }

    async fn save_events(
        &self,
        user_id: UserId,
        events: Vec<UserDomainEvent>,
    ) -> Result<(), UserDomainError> {
        let mut wrapped_events: Vec<EventEnvelope<User>> = Vec::new();
        for payload in events {
            wrapped_events.push(EventEnvelope {
                aggregate_id: *user_id.value(),
                occurred_at: payload.occurred_at(),
                payload,
                metadata: Default::default(),
            })
        }
        self.store.save_events(wrapped_events).await.map_err(
            |event_store_error: EventStoreError| {
                UserDomainError::CouldNotSaveUserEvents(event_store_error.to_string())
            },
        )
    }
}
