use crate::domain::{EventEnvelope, EventSourcedAggregate};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[async_trait]
pub trait EventStoreInterface<A>: Send + Sync
where
    A: EventSourcedAggregate,
{
    async fn load_events(
        &self,
        aggregate_id: &Uuid,
    ) -> Result<Vec<EventEnvelope<A>>, EventStoreError>;
    async fn save_events(&self, events: Vec<EventEnvelope<A>>) -> Result<(), EventStoreError>;
}

#[derive(Debug)]
pub enum EventStoreError {
    DatabaseConnectionError(String),
    SerializationError(String)
}

impl Display for EventStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventStoreError::DatabaseConnectionError(error) => {
                write!(f, "Database problem: {}.", error)
            },
            EventStoreError::SerializationError(error) => {
                write!(f, "Serialization problem, probably serde: {}.", error)
            }
        }
    }
}
impl Error for EventStoreError {}
