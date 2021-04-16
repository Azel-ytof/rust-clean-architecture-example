use std::{error::Error, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct AddUserError {}

impl Error for AddUserError {}

impl fmt::Display for AddUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "A problem occurred while adding a new user...")
    }
}
