use crate::domain::{DomainEvent, UserRegisteredDomainEvent};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserEvent{
    RegisteredUser(UserRegisteredDomainEvent)
}
// I would like to force all variants to have the trait DomainEvent, but I don't think
// it's possible
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
