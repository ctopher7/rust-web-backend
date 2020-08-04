use serde::{Deserialize};
use validator::Validate;

#[derive(Deserialize,Validate)]
pub struct LoginRequestBody{
    #[validate(email)]pub email: String,
    pub password: String,
}