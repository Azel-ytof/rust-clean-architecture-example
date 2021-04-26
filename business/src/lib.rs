use std::error::Error;

pub mod add_user_use_case;

pub trait InputMessage {}

pub trait OutputMessage {}

pub trait InputBoundary<T>
    where
        T: InputMessage,
{
    fn execute(&mut self, message: T);
}

pub trait OutputBoundary<T, V>
    where
        T: OutputMessage,
        V: Error,
{
    fn success(&mut self, message: T);
    fn error(&mut self, message: T, error: V);
}
