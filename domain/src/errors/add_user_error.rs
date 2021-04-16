use std::{error::Error, fmt};
use std::fmt::Formatter;

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

impl Error for AddUserError {}

impl fmt::Display for AddUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = format!("A problem occurred while adding a new user : {}", self.error_message);
        write!(f, "{}", message)
    }
}
