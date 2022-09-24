use chrono::{DateTime, Utc};
use uuid::Uuid;


struct UserReadModel{
    count: u64,
    uuid: Uuid,
    email: String,
    password_hash: String,
    active: bool,
    registered_at: DateTime<Utc>,
    activated_at: Option<DateTime<Utc>>,
    last_login: Option<DateTime<Utc>>,
    successful_login_attempts: u64,
    unsuccessful_login_attempts: u64
}
