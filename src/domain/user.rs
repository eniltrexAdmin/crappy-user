use crate::domain::{UserEmail, UserId, UserPassword};



// Aggregate
pub struct User {
    id: UserId,
    email: UserEmail,
    password: UserPassword
}

