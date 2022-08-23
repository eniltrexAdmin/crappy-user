#[path = "infrastructure/actix.rs"]
pub mod actix;
pub mod application;
#[path = "config/configuration.rs"]
pub mod configuration;
pub mod domain;
#[path = "infrastructure/telemetry/telemetry.rs"]
pub mod telemetry;
// #[path = "infrastructure/persistence/mod.rs"] pub mod persistence;
#[path = "infrastructure/persistence/postgres/event_store_postgres.rs"]
pub mod event_store_postgres;
#[path = "infrastructure/persistence/serialized_events.rs"]
pub mod serialized_event;
