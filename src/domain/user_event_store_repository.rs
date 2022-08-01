use crate::domain::{EventStoreInterface, User, UserDomainError, UserId};


pub struct UserEventStoreRepository<ES>
    where
        ES: EventStoreInterface,
{
    store: ES
}

impl<ES> UserEventStoreRepository<ES>
    where
        ES: EventStoreInterface
{
    async fn load(&self, user_id: UserId) -> Result<User, UserDomainError> {
        let events_to_apply =  self.store.load_events(user_id.value().into()).await?;
        let user = User::default();
        for event in events_to_apply {

        }
    }

}
