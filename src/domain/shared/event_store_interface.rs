use crate::domain::{EventEnvelope, EventSourcedAggregate};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait EventStoreInterface<A>: Send + Sync
where
    A: EventSourcedAggregate,
{
    async fn load_events(&self, aggregate_id: &Uuid) -> Result<Vec<EventEnvelope<A>>, A::Error>;
    async fn save_events(&self, events: Vec<EventEnvelope<A>>) -> Result<(), A::Error>;
}
