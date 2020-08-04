use serde::{Serialize,Deserialize};
use chrono::{Utc,DateTime,naive::NaiveDate};
use validator::Validate;

use crate::regex::user::{PHONE_NUMBER,NAME,USER_ROLE,DATE,validate_password};

#[derive(Serialize)]
pub struct User{
    pub id: i64,
    pub user_role_id: i32,
    pub user_status_id: i32,
    pub last_logged_in: DateTime<Utc>,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Deserialize,Validate)]
pub struct CreateUser{
    #[validate(regex(path = "USER_ROLE"))]pub user_role: String,
    #[validate(email)]pub email: String,
    #[validate(custom = "validate_password")]pub password: String,
    #[validate(regex(path = "PHONE_NUMBER"))]pub phone_number: String,
    #[validate(regex(path = "NAME"))]pub name: Option<String>,
    #[validate(regex(path = "DATE"))]pub date_of_birth: Option<String>,
}