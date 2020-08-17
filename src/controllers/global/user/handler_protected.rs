use actix_web::{
    web::{Data,Json},
    HttpRequest
};
use sqlx::{query,query_as};
use bcrypt::{hash,verify};
use chrono::{Utc};

use crate::utils::{
    http_body::{Message,MessageWithData,get_data_from_middleware},
    error::ApiError,
    validator::validate_input
};

use super::data::{UpdateProfile,GetProfile,ChangePassword};

pub async fn get_profile(
    state:Data<crate::AppState>,
    req:HttpRequest
) ->Result<Json<MessageWithData<GetProfile>>,ApiError>{
    let jwt_decoded = get_data_from_middleware(&req)?;

    let data:GetProfile = query_as!(
        GetProfile,
        "SELECT 
        users.phone_number phone_number,
        users.email email,
        users.date_of_birth date_of_birth,
        users.created_at created_at,
        users.name,
        user_status.status user_status
        FROM users JOIN user_status 
        ON users.user_status_id = user_status.id 
        WHERE users.id = $1;
        ",
        jwt_decoded["id"].as_i64()
    ).fetch_one(&state.db_postgres).await?;

    Ok(Json(MessageWithData{
        msg:"OK",
        data
    }))
}

pub async fn update_profile(
    body:Json<UpdateProfile>,
    state:Data<crate::AppState>,
    req:HttpRequest
)-> Result<Json<Message>,ApiError>{
    validate_input(&body)?; 

    let jwt_decoded = get_data_from_middleware(&req)?;

    let date_of_birth:Option<chrono::NaiveDate> = if let Some(dob) = &body.date_of_birth {
        let splitted:Vec<u32> = dob.split("-").into_iter().map(|x|->u32{
            x.parse::<u32>().unwrap()
        }).collect();

        Some(chrono::naive::NaiveDate::from_ymd(splitted[0] as i32,splitted[1],splitted[2]))
    } else {
        None
    };

    query!(
        "UPDATE users SET 
        name = COALESCE($1,name),
        date_of_birth = COALESCE($2,date_of_birth),
        phone_number = COALESCE($3,phone_number)
        WHERE id = $4;
        ",
        body.name.clone(),date_of_birth,body.phone_number.clone(),jwt_decoded["id"].clone().as_i64()
    ).execute(&state.db_postgres).await?;

    Ok(Json(Message{
        msg:"OK"
    }))
}

pub async fn change_password(
    body:Json<ChangePassword>,
    state:Data<crate::AppState>,
    req:HttpRequest
)->Result<Json<Message>,ApiError>{
    validate_input(&body)?;

    let jwt_decoded = get_data_from_middleware(&req)?;

    let data_user = query!(
        "SELECT id,password FROM users WHERE id = $1;",
        jwt_decoded["id"].as_i64()
    ).fetch_one(&state.db_postgres).await?;

    let verify_old_password = verify(&body.previous_password, data_user.password.as_str())?;

    if !verify_old_password{
        return Err(ApiError::Forbidden("Old password does not match!".to_string()));
    }

    query!(
        "UPDATE users SET password=$1,last_logged_in=$2 WHERE id = $3;",
        hash(&body.new_password, 6u32)?,
        Utc::now(),
        jwt_decoded["id"].as_i64()
    ).execute(&state.db_postgres).await?;

    Ok(Json(Message{
        msg:"OK"
    }))
}