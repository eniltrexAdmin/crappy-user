use crate::domain::UserDomainError;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserEmail(EmailAddress);
impl UserEmail {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        EmailAddress::from_str(value)
            .map(|email_address| UserEmail { 0: email_address })
            .map_err(|error| UserDomainError::InvalidUserEmail(error.to_string()))
    }
    pub fn value(&self) -> String {
        self.0.clone().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn user_email_creation() {
        let value = "francesc.eniltrex@gmail.com";
        assert_ok!(UserEmail::new(value));
    }
    #[test]
    fn user_email_value() {
        let value = "francesc.eniltrex@gmail.com";
        let user_email = UserEmail::new(value);
        assert_eq!(value, user_email.unwrap().value())
    }

    #[test]
    fn invalid_user_email() {
        let value = "francesc.eniltrex.gmail.com";
        let user_email = UserEmail::new(value);
        assert_err!(user_email);
    }
}
