use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct AddUserError {
    error_message: String,
}

impl AddUserError {
    pub fn new(error_message: String) -> Self {
        Self {
            error_message
        }
    }
}

impl std::error::Error for AddUserError {}

impl Display for AddUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let message = format!("A problem occurred while adding a new user : {}", self.error_message);
        write!(f, "{}", message)
    }
}
