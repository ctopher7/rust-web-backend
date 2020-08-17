use actix_web::{
    web::{Json,Data},
    http::{header::SET_COOKIE,HeaderValue},
    HttpResponse,
};
use sqlx::query;
use bcrypt::{verify,hash};
use std::env::var;

use crate::utils::{
    error::ApiError,
    http_body::Message,
    validator::validate_input,
    auth::{create_auth_cookie,new as create_jwt},
    mailer
};
use crate::assets::user_verification_email;

use super::data::{LoginRequestBody,SignUp,CheckEmailExist};

pub async fn web_login(
    body:Json<LoginRequestBody>,
    state: Data<crate::AppState>,
)-> Result<HttpResponse,ApiError>{
    validate_input(&body)?;

    let data_user = query!(
        "SELECT id,password FROM users WHERE email=$1;",
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

pub async fn check_email_exist(
    body:Json<CheckEmailExist>,
    state:Data<crate::AppState>
)-> Result<Json<Message>,ApiError>{
    validate_input(&body)?;

    let data = query!(
        "SELECT id FROM users WHERE email = $1",
        &body.email
    ).fetch_one(&state.db_postgres).await;

    if let Ok(_) = data {
        return Err(ApiError::Conflict("email already exist".to_string()));
    }

    Ok(Json(Message{
        msg:"OK"
    }))
}

pub async fn sign_up(
    body:Json<SignUp>,
    state:Data<crate::AppState>
)->Result<Json<Message>,ApiError>{
    validate_input(&body)?;

    let data_user_exist = query!(
        "SELECT id FROM users WHERE email = $1;",
        &body.email
    ).fetch_one(&state.db_postgres).await;

    if let Ok(_) = data_user_exist {
        return Err(ApiError::Conflict("Email already registered".to_string()));
    }

    let data_user = query!(
        "INSERT INTO users(
            user_role_id,
            user_status_id,
            email,
            phone_number,
            password,
            name,
            last_logged_in
        )
        VALUES (
            (SELECT id FROM user_roles WHERE role = 'customer'),
            (SELECT id FROM user_status WHERE status = 'unverified'),
            $1,$2,$3,$4,$5
        ) RETURNING id;
        ",
        &body.email,
        &body.phone_number,
        hash(&body.password,6)?,
        body.name.clone().unwrap(),
        chrono::Utc::now()
    ).fetch_one(&state.db_postgres).await?;

    let url_to_send = format!("{}{}",var("MAIL_VERIFICATION_URL")?,create_jwt(data_user.id as i64, &state).await?);

    mailer::send(body.email.clone(), "Verfication".to_string(), user_verification_email::create_mail(url_to_send))?;

    Ok(Json(Message{
        msg:"OK"
    }))
}