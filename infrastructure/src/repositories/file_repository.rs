use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use domain::entities::user_entity::UserEntity;
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
    ///
    /// Add user details in file (actually, erasing file content each time this function is called)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use domain::ports::add_user_port::AddUserPort;
    /// use infrastructure::repositories::file_repository::FileRepository;
    ///
    /// let file_path = String::from("tmp/tmp_file_test.csv");
    /// let file_repository = FileRepository::new(file_path.clone());
    ///
    /// let last_name = String::from("Last Name Example");
    /// let first_name = String::from("First Name Example");
    /// let email = String::from("email@example.com");
    /// let result = file_repository.add_user(&last_name, &first_name, &email);
    ///
    /// assert!(Path::new(&file_path).exists())
    ///
    /// ```
    ///
    fn add_user(
        &self,
        last_name: &String,
        first_name: &String,
        email: &String,
    ) -> Result<UserEntity, AddUserError> {
        let content = format!("{};{};{}", last_name, first_name, email);
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
            )),
        }
    }
}
