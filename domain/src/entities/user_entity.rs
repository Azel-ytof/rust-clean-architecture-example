use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug)]
pub struct UserEntity {
    last_name: String,
    first_name: String,
    email: String,
    address: Box<dyn Address>,
}

impl UserEntity {
    pub fn new(last_name: String, first_name: String, email: String, address: Box<dyn Address>) -> Self {
        Self {
            last_name,
            first_name,
            email,
            address,
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

    pub fn get_address(&self) -> &Box<dyn Address> { &self.address }
}

impl Display for UserEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "The user {} {} have email : {}", self.last_name, self.first_name, self.email)
    }
}

// Because content depending on country
pub trait Address: Clone {
    fn to_string(&self) -> String;
}
