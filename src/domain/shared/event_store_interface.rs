use crate::domain::EventEnvelope;

#[async_trait]
pub trait EventStoreInterface: Send + Sync {
    async fn load_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Vec<EventEnvelope<A>>, Err<A::Error>>;
}