use cqrs_es::DomainEvent;
use crate::domain::UserRegisteredDomainEvent;
use serde::{Serialize, Deserialize};

// stupid class. Its convenient to implement aggregate on my user aggregate
// but it requires a domain event type as an enum.
// I need to implement domain event here. but take into consideration the "version"
// of the domain event... it makes no sense, if I change how one event in the enum
// works, I have to change the version? even though all the other domain events inside the
// enum are the exact same???

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserEvent{
    RegisteredUser(UserRegisteredDomainEvent)
}

impl DomainEvent for UserEvent{
    fn event_type(&self) -> String {
        return match self{
            UserEvent::RegisteredUser(event) => {
                event.event_type()
            }
        }
    }

    fn event_version(&self) -> String {
        return match self{
            UserEvent::RegisteredUser(event) => {
                event.event_version()
            }
        }
    }
}
