use cqrs_es::Aggregate;
use crate::domain::{EventStoreInterface, User, UserDomainError, UserId};


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
    async fn load(&self, user_id: UserId) -> Result<User, UserDomainError> {
        let events_to_apply =  self.store.load_events(user_id.value().into()).await?;
        let mut user = User::default();
        for event in events_to_apply {
            user.apply(event.payload)?
        }
        Ok(User)
    }
}
