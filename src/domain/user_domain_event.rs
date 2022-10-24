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

// TODO this is ugly as fuck, how to do this better? ->#[enum_dispatch(DomainEvent)]
