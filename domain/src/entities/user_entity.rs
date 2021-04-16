pub struct UserEntity {
    lastname: String,
    firstname: String,
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
    pub fn new(lastname: String, firstname: String, email: String) -> Self {
        Self {
            lastname,
            firstname,
            email,
        }
    }
}
