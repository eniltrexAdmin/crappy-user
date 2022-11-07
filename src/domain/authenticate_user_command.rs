use secrecy::SecretString;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeSeq;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct AuthenticateUserCommand {
    pub id: Uuid,
    pub email: String,
    pub password_attempt: SecretString,
}
impl AuthenticateUserCommand {
    pub fn new(id: Uuid, email: String, password_attempt: SecretString) -> Self {
        Self {
            id,
            email,
            password_attempt,
        }
    }
}

impl Serialize for AuthenticateUserCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.email)?;
        seq.serialize_element(&self.id)?;
        seq.end()
    }
}