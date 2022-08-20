use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::domain::EventSourcedAggregate;

pub trait DomainEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
{
    /// A name specifying the event, used for event upcasting.
    fn event_type(&self) -> String;
    /// A version of the `event_type`, used for event upcasting.
    fn event_version(&self) -> String;
}

// copy from cqrs-es crate. But it's very similar to Garofolo book:
// a uuid, a strem name (which would be aggregateName-Uuid) then metadata and then payload.
// I am changing the sequence number ot occurred_at
pub struct EventEnvelope<A>
where
    A: EventSourcedAggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: Uuid,
    /// The sequence number for an aggregate instance.
    pub occurred_at: usize,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
    /// The event payload with all business information.
    pub payload: A::Event,
}
