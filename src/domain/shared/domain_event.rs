use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;
use enum_dispatch::enum_dispatch;

use crate::domain::EventSourcedAggregate;

#[enum_dispatch]
pub trait DomainEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
{
    /// A name specifying the event, used for event upcasting.
    fn event_type(&self) -> String;
    /// A version of the `event_type`, used for event upcasting.
    fn event_version(&self) -> String;

    fn occurred_at(&self) -> DateTime<Utc>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventEnvelope<A>
where
    A: EventSourcedAggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: Uuid,
    /// adding this one
    pub occurred_at: DateTime<Utc>,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
    /// The event payload with all business information.
    pub payload: A::Event,
}
