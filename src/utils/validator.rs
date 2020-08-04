use crate::utils::error::ApiError;
use actix_web::web::Json;
use validator::{Validate};
use serde_json::{json,value::Value};

pub fn validate_input<T:Validate>(
    params: &Json<T>
) -> Result<(), ApiError>{
    match params.validate() {
        Ok(()) => Ok(()),
        Err(error) => Err(ApiError::BadRequest( Value::to_string(&json!(error.field_errors()))))
    }
}