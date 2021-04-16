pub mod add_user_use_case;

use domain::errors::add_user_error::AddUserError;

pub trait InputMessage {}
pub trait OutputMessage {}

pub trait InputBoundary<T>
    where
        T: InputMessage,
{
    fn execute(&mut self, message: T);
}

pub trait OutputBoundary<T>
    where
        T: OutputMessage,
{
    fn success(&mut self, message: &T);
    fn error(&mut self, message: &T, error: AddUserError);
}
