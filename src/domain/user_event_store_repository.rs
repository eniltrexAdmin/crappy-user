use chrono::Utc;
use crate::domain::{
    EventEnvelope, EventSourcedAggregate, EventStoreError, EventStoreInterface, User,
    UserDomainError, UserEvent, UserId,
};

pub struct UserEventStoreRepository<ES>
where
    ES: EventStoreInterface<User>,
{
    pub store: ES,
}

impl<ES> UserEventStoreRepository<ES>
where
    ES: EventStoreInterface<User>,
{
    pub async fn load(&self, user_id: UserId) -> Result<User, UserDomainError> {
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

    pub async fn save_events(
        &self,
        user_id: UserId,
        events: Vec<UserEvent>,
    ) -> Result<(), UserDomainError> {
        let mut wrapped_events: Vec<EventEnvelope<User>> = Vec::new();
        for payload in events {
            wrapped_events.push(EventEnvelope {
                aggregate_id: *user_id.value(),
                occurred_at: Utc::now(),
                payload,
                metadata: Default::default(),
            })
        }
        self.store.save_events(wrapped_events)
            .await
            .map_err(|event_store_error: EventStoreError| {
                UserDomainError::CouldNotSaveUserEvents(event_store_error.to_string())
            })
    }
}
