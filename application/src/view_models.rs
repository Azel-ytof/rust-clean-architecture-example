pub mod add_user_view_model;

pub trait ViewModel {
    fn is_success(&self) -> bool;
    fn is_error(&self) -> bool;
}
