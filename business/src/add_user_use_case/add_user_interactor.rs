use domain::ports::add_user_port::AddUserPort;

use crate::{InputBoundary, OutputBoundary};
use crate::add_user_use_case::add_user_input_message::AddUserInputMessage;
use crate::add_user_use_case::add_user_output_message::AddUserOutputMessage;
use domain::errors::add_user_error::AddUserError;

pub struct AddUserInteractor {
    presenter: Box<dyn OutputBoundary<AddUserOutputMessage, AddUserError>>,
    add_user_repository: Box<dyn AddUserPort>,
    output_message: AddUserOutputMessage,
}

impl AddUserInteractor {
    pub fn new(presenter: Box<dyn OutputBoundary<AddUserOutputMessage, AddUserError>>, add_user_repository: Box<dyn AddUserPort>) -> Self {
        Self {
            presenter,
            add_user_repository,
            output_message: AddUserOutputMessage::new(),
        }
    }
}

impl InputBoundary<AddUserInputMessage> for AddUserInteractor
{
    fn execute(&mut self, message: AddUserInputMessage) {
        let last_name = message.get_last_name();
        let first_name = message.get_first_name();
        let email = message.get_email();

        let result_user = self.add_user_repository.add_user(&last_name, &first_name, &email);

        match result_user {
            Ok(u) => self.output_message.set_user(u),
            Err(e) => {
                self.presenter.error(self.output_message.clone(), e);
                return;
            }
        }

        self.presenter.success(self.output_message.clone());
    }
}
