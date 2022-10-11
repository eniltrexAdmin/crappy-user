use crate::domain::UserDomainError;
use argon2::Argon2;
use password_hash::{PasswordHash, SaltString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserPassword {
    pub hash_string: String,
    pub salt: String,
}
impl UserPassword {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let salt_value = SaltString::generate(thread_rng());
        let hash = PasswordHash::generate(Argon2::default(), value, salt_value.as_str())
            .map_err(|error| UserDomainError::CouldNotGeneratePassword(error.to_string()))?;

        Ok(UserPassword {
            hash_string: hash.to_string(),
            salt: salt_value.to_string(),
        })
    }

    pub fn from_storage(hash_string: String, salt: String) -> Self {
        UserPassword { hash_string, salt }
    }

    pub fn validate_password(password_attempt: &str) -> Result<(),>
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};
    use password_hash::PasswordVerifier;

    #[test]
    fn generate_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password);
        assert_ok!(user_password);
    }

    #[test]
    fn retrieve_saved_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();
        let salt = user_password.salt.clone();
        let password_hash = user_password.hash_string.clone();
        let retrieved_user_password = UserPassword::from_storage(password_hash, salt.to_string());

        assert_eq!(
            retrieved_user_password.hash_string,
            user_password.hash_string
        );
    }

    #[test]
    fn validate_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();

        let password_attempt = "secret_password";
        let password_hash = PasswordHash::new(&user_password.hash_string).unwrap();

        let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];
        assert_ok!(password_hash.verify_password(alg, password_attempt));
    }

    #[test]
    fn invalid_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();

        let password_attempt = "bad_password";
        let password_hash = PasswordHash::new(&user_password.hash_string).unwrap();

        let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];
        assert_err!(password_hash.verify_password(alg, password_attempt));
    }
}
