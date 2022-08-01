use uuid::Uuid;
use crate::domain::{EventEnvelope, EventSourcedAggregate};

#[async_trait]
pub trait EventStoreInterface<A>: Send + Sync
    where
        A: EventSourcedAggregate{
    async fn load_events(
        &self,
        aggregate_id: Uuid,
    ) -> Result<Vec<EventEnvelope<A>>, Err<A::Error>>;
}