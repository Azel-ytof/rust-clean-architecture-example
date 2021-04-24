use std::rc::Rc;

use domain::entities::user_entity::UserEntity;

use crate::OutputMessage;

#[derive(Clone)]
pub struct AddUserOutputMessage {
    user: Option<Rc<UserEntity>>
}

impl AddUserOutputMessage {
    pub fn new() -> Self {
        Self {
            user: None
        }
    }

    pub fn get_user(self) -> Option<Rc<UserEntity>> {
        self.user
    }

    pub fn set_user(&mut self, user: UserEntity) {
        self.user = Some(Rc::new(user));
    }
}

impl OutputMessage for AddUserOutputMessage {}
