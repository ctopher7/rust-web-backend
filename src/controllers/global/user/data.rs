use serde::{Deserialize,Serialize};
use validator::Validate;
use chrono::{DateTime,Utc,NaiveDate};

use crate::regex::user::{PHONE_NUMBER,NAME,validate_password,DATE};

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

#[derive(Deserialize,Validate)]
pub struct CheckEmailExist{
    #[validate(email)]pub email: String
}

#[derive(Deserialize,Validate)]
pub struct UpdateProfile{
    #[validate(regex(path = "PHONE_NUMBER"))]pub phone_number: Option<String>,
    #[validate(regex(path = "NAME"))]pub name: Option<String>,
    #[validate(regex(path = "DATE"))]pub date_of_birth: Option<String>,
}

#[derive(Serialize)]
pub struct GetProfile{
    pub phone_number: String,
    pub email: String,
    pub user_status: String,
    pub name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: DateTime<Utc>
}

#[derive(Deserialize,Validate)]
pub struct ChangePassword{
    pub previous_password:String,
    #[validate(custom = "validate_password")]pub new_password: String
}