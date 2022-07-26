use crate::domain::{UserEmail, UserId, UserPassword};

pub struct User {
    id: UserId,
    email: Option<UserEmail>,
    password: Option<UserPassword>,
    is_registered: bool
}

impl User {
    pub fn new(id: UserId) -> Self {
        User {
            id,
            email: None,
            password:None,
            is_registered: false
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn email(self) -> Option<UserEmail> {
        self.email
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;
    #[test]
    fn new_user() {
        let user_id = UserId::new(Uuid::new_v4());
        let user = User::new(user_id.clone());
        assert_eq!(user.id(), user_id);
        assert_eq!(false, user.is_registered());
        assert_eq!(None, user.email());
    }
}


