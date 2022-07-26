#[path = "infrastructure/actix.rs"]
pub mod actix;
#[path = "config/configuration.rs"]
pub mod configuration;
#[path = "infrastructure/telemetry/telemetry.rs"]
pub mod telemetry;
pub mod domain;
pub mod application;
// #[path = "infrastructure/persistence/mod.rs"] pub mod persistence;