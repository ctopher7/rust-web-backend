use serde::{Serialize,Deserialize};
use actix_web::HttpRequest;
use serde_json::Value as JsonValue;
use super::error::ApiError;

#[derive(Deserialize,Serialize)]
pub struct MessageWithData<T> {
    pub msg: &'static str,
    pub data: T
}

#[derive(Deserialize,Serialize)]
pub struct Message {
    pub msg: &'static str
}

pub fn get_data_from_middleware(req:&HttpRequest)->Result<JsonValue,ApiError>{
    let header_extension = req.head().extensions();
    Ok(header_extension.get::<JsonValue>().unwrap().clone())
}