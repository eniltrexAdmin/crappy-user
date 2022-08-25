use crate::domain::{DomainEvent, EventEnvelope, EventSourcedAggregate, EventStoreError};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct SerializedEvent {
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub event_type: String,
    pub event_version: String,
    pub payload: Value,
    pub metadata: Value,
    #[sqlx(rename = "timestamp")]
    pub occurred_at: DateTime<Utc>,
}

impl SerializedEvent {
    /// Create a new SerializedEvent with the given values.
    pub fn new(
        aggregate_type: String,
        aggregate_id: Uuid,
        event_type: String,
        event_version: String,
        payload: Value,
        metadata: Value,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        Self {
            aggregate_type,
            aggregate_id,
            event_type,
            event_version,
            payload,
            metadata,
            occurred_at,
        }
    }
}

pub(crate) fn serialize_events<A: EventSourcedAggregate>(
    events: &[EventEnvelope<A>],
) -> Result<Vec<SerializedEvent>, EventStoreError> {
    let mut result: Vec<SerializedEvent> = Default::default();
    for event in events {
        result.push(SerializedEvent::try_from(event)?);
    }
    Ok(result)
}

impl<A: EventSourcedAggregate> TryFrom<&EventEnvelope<A>> for SerializedEvent {
    type Error = EventStoreError;

    fn try_from(event: &EventEnvelope<A>) -> Result<Self, Self::Error> {
        let aggregate_type = A::aggregate_type();
        let event_type = event.payload.event_type();
        let event_version = event.payload.event_version();
        let payload = serde_json::to_value(&event.payload)?;
        let metadata = serde_json::to_value(&event.metadata)?;
        Ok(Self {
            aggregate_id: event.aggregate_id,
            aggregate_type,
            event_type,
            event_version,
            payload,
            metadata,
            occurred_at: event.occurred_at,
        })
    }
}

impl<A: EventSourcedAggregate> TryFrom<SerializedEvent> for EventEnvelope<A> {
    type Error = EventStoreError;

    fn try_from(serialized_event: SerializedEvent) -> Result<Self, Self::Error> {
        // impl from serde in this error is done in event_store_interface.
        let payload = serde_json::from_value(serialized_event.payload)?;
        let metadata = serde_json::from_value(serialized_event.metadata)?;
        Ok(EventEnvelope {
            aggregate_id: serialized_event.aggregate_id,
            occurred_at: serialized_event.occurred_at,
            payload,
            metadata,
        })
    }
}
