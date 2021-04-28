use std::rc::Rc;

use domain::entities::user_entity::UserEntity;
use domain::errors::add_user_error::AddUserError;

use crate::view_models::ViewModel;

pub struct AddUserViewModel {
    user: Option<Rc<UserEntity>>,
    error: Option<AddUserError>,
}

impl AddUserViewModel {
    pub fn new(user: Option<Rc<UserEntity>>, error: Option<AddUserError>) -> Self {
        Self {
            user,
            error,
        }
    }

    pub fn get_user(&self) -> Option<&Rc<UserEntity>> {
        self.user.as_ref()
    }

    pub fn get_error(&self) -> Option<&AddUserError> {
        self.error.as_ref()
    }
}

impl ViewModel for AddUserViewModel {
    fn is_success(&self) -> bool {
        self.error.is_none()
    }

    fn is_error(&self) -> bool {
        self.error.is_some()
    }
}
