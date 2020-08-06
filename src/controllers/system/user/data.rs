use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UserVerificationQuery{
    pub token: String
}