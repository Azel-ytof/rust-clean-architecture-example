use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug)]
pub struct UserEntity {
    last_name: String,
    first_name: String,
    email: String,
}

impl UserEntity {
    /// Create a new UserEntity
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::entities::user_entity::UserEntity;
    ///
    /// let last_name = String::from("Last Name Example");
    /// let first_name = String::from("First Name Example");
    /// let email = String::from("email@example.com");
    ///
    /// let user = UserEntity::new(last_name, first_name, email);
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

impl Display for UserEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "The user {} {} have email : {}", self.last_name, self.first_name, self.email)
    }
}
