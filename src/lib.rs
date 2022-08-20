#[path = "infrastructure/actix.rs"]
pub mod actix;
pub mod application;
#[path = "config/configuration.rs"]
pub mod configuration;
pub mod domain;
#[path = "infrastructure/telemetry/telemetry.rs"]
pub mod telemetry;
// #[path = "infrastructure/persistence/mod.rs"] pub mod persistence;
#[path = "infrastructure/persistence/event_store/event_store.rs"]
pub mod event_store;
