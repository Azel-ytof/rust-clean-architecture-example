use business::add_user_use_case::add_user_output_message::AddUserOutputMessage;
use business::OutputBoundary;
use domain::errors::add_user_error::AddUserError;

use crate::view_models::add_user_view_model::AddUserViewModel;

pub struct AddUserPresenter {
    view_model: Option<AddUserViewModel>
}

impl AddUserPresenter {
    pub fn new() -> Self {
        Self {
            view_model: None
        }
    }
}

impl OutputBoundary<AddUserOutputMessage, AddUserError> for AddUserPresenter {
    fn success(&mut self, message: AddUserOutputMessage) {
        let view_model = AddUserViewModel::new(message.get_user(), None);
        self.view_model = Some(view_model);
    }

    fn error(&mut self, message: AddUserOutputMessage, error: AddUserError) {
        let view_model = AddUserViewModel::new(message.get_user(), Some(error));
        self.view_model = Some(view_model);
    }
}
