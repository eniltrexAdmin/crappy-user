use crate::domain::UserDomainError;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
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
        let hash = PasswordHash::generate(Self::get_encryption_algorithm(), value, salt_value.as_str())
            .map_err(|error| UserDomainError::CouldNotGeneratePassword(error.to_string()))?;

        Ok(UserPassword {
            hash_string: hash.to_string(),
            salt: salt_value.to_string(),
        })
    }

    pub fn get_encryption_algorithm() -> impl PasswordHasher + PasswordVerifier{
        Argon2::default()
    }

    pub fn verify_password(&self, password_attempt: &str) -> Result<(), UserDomainError> {
        let alg = &UserPassword::get_encryption_algorithm();
        let alg: Vec<&dyn PasswordVerifier> = vec![alg];
        let password_hash = PasswordHash::new(&self.hash_string).unwrap();
        password_hash.verify_password(&alg, password_attempt)
            .map_err(|_| UserDomainError::IncorrectPassword)
    }

    pub fn from_storage(hash_string: String, salt: String) -> Self {
        UserPassword { hash_string, salt }
    }

    pub fn clone(&self) -> Self {
        Self{
            hash_string: self.hash_string.to_string(),
            salt: self.salt.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn generate_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password);
        assert_ok!(user_password);
    }

    #[test]
    fn clone() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();
        let cloned_password = user_password.clone();
        assert_eq!(user_password, cloned_password);
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
        assert_eq!(
            retrieved_user_password.salt,
            user_password.salt
        );
    }

    #[test]
    fn validate_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();
        let password_attempt = "secret_password";

        assert_ok!(user_password.verify_password(password_attempt));
    }

    #[test]
    fn invalid_password() {
        let password = "secret_password";
        let user_password = UserPassword::new(password).unwrap();
        let password_attempt = "bad_password";

        assert_err!(user_password.verify_password(password_attempt));
    }
}
