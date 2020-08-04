use regex::Regex;
use lazy_static::lazy_static;
use fancy_regex::{Regex as FancyRegex};
use validator::ValidationError;

lazy_static! {
    pub static ref PHONE_NUMBER: Regex = Regex::new(r"^(08|\+)[0-9]{7,16}$").unwrap();
    pub static ref NAME: Regex = Regex::new(r"[a-zA-Z0-9 ]{3,30}$").unwrap();
    pub static ref ADDRESS: Regex = Regex::new(r"[a-zA-Z0-9 ,.]{8,50}$").unwrap();
    static ref PASSWORD: FancyRegex = FancyRegex::new(r#"^(?=.{6,30})(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*()_+=/\-|\\{}\[\]\:\;\"\',.<>?`~]).*$"#).unwrap();
    pub static ref USER_ROLE: Regex = Regex::new(r#"(\badmin\b|\bcustomer\b)"#).unwrap();
    pub static ref DATE: Regex = Regex::new(r#"\d{4}-(0?[1-9]|1[0-2])-([0-2]?[1-9]|[1-3][01])$"#).unwrap();
}

pub fn validate_password(password:&str)-> Result<(), ValidationError>{
    let check = PASSWORD.is_match(password).unwrap();

    if check == false{
        return Err(ValidationError::new("Password requirement not satisfied"));
    }

    Ok(())
}