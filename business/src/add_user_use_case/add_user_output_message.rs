use domain::entities::user_entity::UserEntity;

use crate::OutputMessage;

pub struct AddUserOutputMessage {
    user: Option<UserEntity>
}

impl AddUserOutputMessage {
    pub fn new() -> Self {
        Self {
            user: None
        }
    }

    pub fn get_user(self) -> Option<UserEntity> {
        self.user
    }

    pub fn set_user(&mut self, user: UserEntity) {
        self.user = Some(user);
    }
}

impl OutputMessage for AddUserOutputMessage {}
