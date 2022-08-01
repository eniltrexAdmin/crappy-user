use crate::domain::DomainEvent;

// Default is needed for a "default" empty state. This is also in the book
// to have an "init" state.
#[async_trait]
pub trait EventSourcedAggregate: Default + Sync + Send {
    type Event: DomainEvent;
    type Error: std::error::Error;
    fn apply(&mut self, event: Self::Event);
    // not handle.
}