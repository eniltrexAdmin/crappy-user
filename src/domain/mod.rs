// SHARED
#[path = "shared/event_sourced_aggregate.rs"]
pub(crate) mod event_sourced_aggregate;
pub use event_sourced_aggregate::*;
#[path = "shared/domain_event.rs"]
mod domain_event;
pub use domain_event::*;
#[path = "shared/event_store_interface.rs"]
mod event_store_interface;
pub use event_store_interface::*;
// AGGREGATE
mod user;
pub use user::*;
mod user_event_store_repository;
pub use user_event_store_repository::*;
mod user_domain_error;
pub use user_domain_error::*;
// DOMAIN EVENTS
mod user_domain_event;
pub use user_domain_event::*;
mod user_registered_event;
pub use user_registered_event::*;
mod user_successfully_authenticated_event;
pub use user_successfully_authenticated_event::*;
mod user_authentication_failed_event;
pub use user_authentication_failed_event::*;
// COMMANDS
mod register_user_command;
pub use register_user_command::*;
mod authenticate_user_command;
pub use authenticate_user_command::*;
// VALUE OBJECTS
mod user_id;
pub use user_id::*;
mod user_email;
pub use user_email::*;
mod user_password;
pub use user_password::*;
// VIEW MODEL
mod user_view_repository;
pub use user_view_repository::*;
// VIEWS
mod user_credentials_view;
pub use user_credentials_view::*;
mod user_read_model;
pub use user_read_model::*;
