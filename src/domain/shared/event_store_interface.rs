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
    async fn load_all_events(
        &self,
        last_event_read: i64
    ) -> Result<Vec<EventEnvelope<A>>, EventStoreError>;
}

#[derive(Debug)]
pub enum EventStoreError {
    DatabaseConnectionError(String),
    SerializationError(String),
}

impl Display for EventStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventStoreError::DatabaseConnectionError(error) => {
                write!(f, "Database problem: {}.", error)
            }
            EventStoreError::SerializationError(error) => {
                write!(f, "Serialization problem, probably serde: {}.", error)
            }
        }
    }
}
impl Error for EventStoreError {}

impl From<serde_json::Error> for EventStoreError {
    fn from(err: serde_json::Error) -> Self {
        EventStoreError::SerializationError(err.to_string())
    }
}
