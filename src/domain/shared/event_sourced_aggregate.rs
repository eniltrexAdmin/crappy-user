use crate::domain::DomainEvent;
use async_trait::async_trait;

// Default is needed for a "default" empty state. This is also in the book
// to have an "init" state.
#[async_trait]
pub trait EventSourcedAggregate: Default + Sync + Send {
    type Event: DomainEvent;
    type Error: std::error::Error;
    fn recorded_events(&self) -> Vec<Self::Event>;
    fn aggregate_type() -> String;
    fn apply(&mut self, event: Self::Event);
}
