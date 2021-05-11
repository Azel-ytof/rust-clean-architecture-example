use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use domain::entities::user_entity::{UserEntity, Address};
use domain::errors::add_user_error::AddUserError;
use domain::ports::add_user_port::AddUserPort;

pub struct FileRepository {
    file_path: String,
}

impl FileRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl AddUserPort for FileRepository {
    fn add_user(
        &self,
        last_name: &String,
        first_name: &String,
        email: &String,
        address: &Box<dyn Address>,
    ) -> Result<UserEntity, AddUserError> {
        let content = format!("{};{};{};{}", last_name, first_name, email, address.to_string());
        let file_path = Path::new(&self.file_path);

        match file_path.parent() {
            Some(d) => match std::fs::create_dir_all(d) {
                Err(e) => {
                    let error_message = format!(
                        "An error occured while creating directories {} : {}",
                        d.display(),
                        e
                    );
                    let error = AddUserError::new(String::from(error_message));
                    return Err(error);
                }
                _ => (),
            },
            None => println!("No parents directories to create"),
        }

        let display = file_path.display();

        let mut file = match File::create(&file_path) {
            Err(e) => {
                let error_message =
                    format!("An error occured while creating file {} : {}", display, e);
                let error = AddUserError::new(String::from(error_message));
                return Err(error);
            }
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(e) => {
                let error_message = format!(
                    "An error occured while puting content '{}' in file {} : {}",
                    content, display, e
                );
                let error = AddUserError::new(String::from(error_message));
                return Err(error);
            }
            Ok(_) => Ok(UserEntity::new(
                last_name.clone(),
                first_name.clone(),
                email.clone(),
                address.clone(),
            )),
        }
    }
}
