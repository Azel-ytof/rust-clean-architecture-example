use crate::entities::user_entity::UserEntity;

pub trait AddUserPort{
    fn add_user(&self, lastname: &String, firstname: &String, email: &String) -> UserEntity;
}