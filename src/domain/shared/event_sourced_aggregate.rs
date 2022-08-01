use crate::domain::DomainEvent;

#[async_trait]
pub trait EventSourcedAggregate {
    type Event: DomainEvent;
    type Error: std::error::Error;
}