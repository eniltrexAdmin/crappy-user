use chrono::{DateTime, Utc};
use crate::domain::{DomainEvent, UserAuthenticationFailed, UserRegisteredDomainEvent, UserSuccessfullyAuthenticated};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserDomainEvent {
    RegisteredUser(UserRegisteredDomainEvent),
    UserAuthenticated(UserSuccessfullyAuthenticated),
    UserAuthenticationFailed(UserAuthenticationFailed)
}

// TODO this is ugly as fuck, how to do this better?
impl DomainEvent for UserDomainEvent {
    fn event_type(&self) -> String {
        return match self{
            UserDomainEvent::RegisteredUser(event) => {
                event.event_type()
            },
            UserDomainEvent::UserAuthenticationFailed(event) => {
                event.event_type()
            },
            UserDomainEvent::UserAuthenticated(event) => {
                event.event_type()
            }
        }
    }

    fn event_version(&self) -> String {
        return match self{
            UserDomainEvent::RegisteredUser(event) => {
                event.event_version()
            },
            UserDomainEvent::UserAuthenticationFailed(event) => {
                event.event_version()
            },
            UserDomainEvent::UserAuthenticated(event) => {
                event.event_version()
            }
        }
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        return match self{
            UserDomainEvent::RegisteredUser(event) => {
                event.occurred_at()
            },
            UserDomainEvent::UserAuthenticationFailed(event) => {
                event.occurred_at()
            },
            UserDomainEvent::UserAuthenticated(event) => {
                event.occurred_at()
            }
        }
    }
}
