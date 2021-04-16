use crate::{InputBoundary, OutputBoundary};
use crate::add_user_use_case::add_user_output_message::AddUserOutputMessage;
use crate::add_user_use_case::add_user_input_message::AddUserInputMessage;
use domain::ports::add_user_port::AddUserPort;

pub struct AddUserInteractor {
    presenter: Box<dyn OutputBoundary<AddUserOutputMessage>>,
    add_user_repository: Box<dyn AddUserPort>,
    output_message: AddUserOutputMessage,
}

impl AddUserInteractor {
    pub fn new(presenter: Box<dyn OutputBoundary<AddUserOutputMessage>>, add_user_repository: Box<dyn AddUserPort>) -> Self {
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

        let user = self.add_user_repository.add_user(&last_name, &first_name, &email);

        self.output_message.set_user(user);

        self.presenter.success(&self.output_message);
    }
}