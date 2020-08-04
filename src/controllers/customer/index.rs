use actix_web::{
    web::{Json}
};
use crate::utils::{
    http_body::Message,
    error::ApiError
};

pub async fn main()->Result<Json<Message>,ApiError>{
    Ok(Json(Message{
        msg:"OK"
    }))
}