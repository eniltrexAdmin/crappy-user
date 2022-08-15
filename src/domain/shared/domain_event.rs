use std::collections::HashMap;
use std::fmt;
use serde::de::DeserializeOwned;
use serde::Serialize;
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

// copy from cqrs-es crate. Keeping the exact same since I am planning
// to reuse also the event store repository to query from the event store.
// HESITATING doesn't this belong to infra concerns?
pub struct EventEnvelope<A>
    where
        A: EventSourcedAggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: Uuid,
    /// The sequence number for an aggregate instance.
    pub sequence: usize,
    /// The event payload with all business information.
    pub payload: A::Event,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
}
