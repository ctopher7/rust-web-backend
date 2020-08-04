use actix_web::{
    web::{Json,Data},
    http::{header::SET_COOKIE,HeaderValue},
    HttpResponse,
};
use sqlx::query;
use bcrypt::{verify};

use crate::utils::{
    error::ApiError,
    // http_body::Message,
    validator::validate_input,
    auth::create_auth_cookie
};

use super::data::{LoginRequestBody};

pub async fn login(
    body:Json<LoginRequestBody>,
    state: Data<crate::AppState>,
)-> Result<HttpResponse,ApiError>{
    // println!("{:?}",req.head().extensions().get::<JsonValue>());
    validate_input(&body)?;

    let data_user = query!(
        r#"SELECT id,password FROM users WHERE email=$1;"#,
        &body.email
    ).fetch_one(&state.db_postgres).await;

    if let Err(_) = data_user{
        return Err(ApiError::NotFound("Email not registered".to_string()));
    }

    let data_user_unwrapped = data_user?;

    if verify(&body.password, &data_user_unwrapped.password)? == false {
        return Err(ApiError::Unauthorized("Wrong password".to_string()));
    }

    let auth_cookie = create_auth_cookie(data_user_unwrapped.id, &state).await;
    
    Ok(HttpResponse::Ok().set_header(
        SET_COOKIE, 
        HeaderValue::from_str(&auth_cookie)?
    ).finish())
}