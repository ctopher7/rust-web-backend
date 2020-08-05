use serde::{Deserialize};
use validator::Validate;
use crate::regex::user::{PHONE_NUMBER,NAME,validate_password};

#[derive(Deserialize,Validate)]
pub struct LoginRequestBody{
    #[validate(email)]pub email: String,
    pub password: String,
}

#[derive(Deserialize,Validate)]
pub struct SignUp{
    #[validate(email)]pub email: String,
    #[validate(custom = "validate_password")]pub password: String,
    #[validate(regex(path = "PHONE_NUMBER"))]pub phone_number: String,
    #[validate(regex(path = "NAME"))]pub name: Option<String>,
}