use crate::domain::{EventEnvelope, EventSourcedAggregate, EventStoreInterface, User, UserDomainError, UserEvent, UserId};


pub struct UserEventStoreRepository<ES>
    where
        ES: EventStoreInterface<User>,
{
    store: ES
}

impl<ES> UserEventStoreRepository<ES>
    where
        ES: EventStoreInterface<User>
{
    pub async fn load(&self, user_id: UserId) -> Result<User, UserDomainError> {
        let events_to_apply =  self.store.load_events(user_id.value()).await?;
        let mut user = User::default();
        for event in events_to_apply {
            user.apply(event.payload)
        }
        Ok(user)
    }

    pub async fn save_events(&self, user_id: UserId, events: Vec<UserEvent>) -> Result<(), UserDomainError> {
        let mut wrapped_events: Vec<EventEnvelope<User>> = Vec::new();

        for payload in events {
            wrapped_events.push(EventEnvelope{
                aggregate_id: *user_id.value(),
                sequence: 0,
                payload,
                metadata: Default::default()
            })
        }
        self.store.save_events(wrapped_events).await
        // For some reason this does not compile: it complains about the type missmatch
        // even though it's the exact same type...
        // self.store.save_events(events.iter().map(|event|{
        //     EventEnvelope{
        //         aggregate_id: Default::default(),
        //         sequence: 0,
        //         payload: event,
        //         metadata: Default::default()
        //     }
        // }).collect()).await
    }
}


