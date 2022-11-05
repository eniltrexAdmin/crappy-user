use chrono::{DateTime, Utc};
use crate::domain::{DomainEvent, UserAuthenticationFailed, UserRegisteredDomainEvent, UserSuccessfullyAuthenticated};
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[enum_dispatch(DomainEvent)]
pub enum UserDomainEvent {
    RegisteredUser(UserRegisteredDomainEvent),
    UserAuthenticated(UserSuccessfullyAuthenticated),
    UserAuthenticationFailed(UserAuthenticationFailed)
}

//#[enum_dispatch(DomainEvent)] sacing the day
