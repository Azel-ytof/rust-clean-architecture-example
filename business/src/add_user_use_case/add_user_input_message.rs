use crate::InputMessage;

pub struct AddUserInputMessage {
    last_name: String,
    first_name: String,
    email: String,
}

impl AddUserInputMessage {
    /// Create a new AddUserInputMessage
    ///
    /// # Examples
    ///
    /// ```
    /// use business::add_user_use_case::add_user_input_message::AddUserInputMessage;
    ///
    /// let last_name = String::from("Last Name Example");
    /// let first_name = String::from("First Name Example");
    /// let email = String::from("email@example.com");
    ///
    /// let input_message = AddUserInputMessage::new(last_name, first_name, email);
    /// ```
    ///
    pub fn new(last_name: String, first_name: String, email: String) -> Self {
        Self {
            last_name,
            first_name,
            email,
        }
    }

    pub fn get_last_name(&self) -> &String {
        &self.last_name
    }

    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }
}

impl InputMessage for AddUserInputMessage {}
