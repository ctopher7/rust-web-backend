use actix_web::{
    cookie::{Cookie,SameSite},
    web::Data,
};
use std::{
    ops::Add,
    env::var
};
use jsonwebtoken::{encode, decode as dec, Header, Validation, EncodingKey, DecodingKey,errors::ErrorKind::{InvalidToken,InvalidSignature,ExpiredSignature}};
use serde::{Serialize, Deserialize};
use sqlx::{query,query_as};
use chrono::{DateTime,Utc};

use super::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i64,
    pub exp: i64,
    iat:i64,
    iss:String
}

pub async fn new (id:i64,state:&Data<crate::AppState>)-> Result<String,ApiError>{
    let now =chrono::Utc::now();
    let token = encode(&Header::default(), &Claims{
        id,
        exp: now.timestamp() + var("SESSION_LENGTH")?.parse::<i64>().unwrap(),
        iat: now.timestamp(),
        iss: var("JWT_ISSUER")?
    }, &EncodingKey::from_secret(var("JWT_SECRET")?.as_ref()))?;
    
    query!(
        "UPDATE users SET last_logged_in = $1 where id = $2;",
        now,id
    ).execute(&state.db_postgres).await?;

    Ok(token)
}

pub async fn create_auth_cookie(id:i64,state:&Data<crate::AppState>)->String{
    let mut auth_cookie = Cookie::new("Authorization", new(id, state).await.unwrap());
    if var("ENV").unwrap() == "production".to_string() {auth_cookie.set_secure(true);}
    auth_cookie.set_same_site(SameSite::Strict);
    auth_cookie.set_http_only(true);
    auth_cookie.set_expires(time::OffsetDateTime::now_utc().add(time::Duration::seconds(var("SESSION_LENGTH").unwrap().parse::<i64>().unwrap())));
    auth_cookie.set_path("/");
    auth_cookie.to_string()
}

pub struct User{
    last_logged_in:DateTime<Utc>,
    user_role:String,
    status:String,
}

pub async fn decode_and_authenticate (token: &str,state:&Data<crate::AppState>) ->  Result<(Claims,User),ApiError>{
    match dec::<Claims>(token, &DecodingKey::from_secret(var("JWT_SECRET")?.as_ref()), &Validation::default()) {
        Ok(data) =>{ 
            let data_user:User = query_as!(
                User,
                "SELECT 
                user_status.status status,
                users.last_logged_in last_logged_in, 
                user_roles.role user_role 
                FROM users 
                JOIN user_roles ON users.user_role_id = user_roles.id 
                JOIN user_status ON users.user_status_id = user_status.id
                WHERE users.id = $1;",
                data.claims.id
            ).fetch_one(&state.db_postgres).await?;

            if data_user.last_logged_in.timestamp() > data.claims.iat || Utc::now().timestamp() > data.claims.exp { 
                return Err(ApiError::Unauthorized("Session Expired".to_string())); 
            }

            Ok((data.claims,data_user))
        },
        Err(e) => match e.kind(){
            InvalidToken | InvalidSignature => Err(ApiError::BadRequest("Invalid Token".to_string())),
            ExpiredSignature => Err(ApiError::Unauthorized("Session Expired".to_string())),
            _ => Err(ApiError::InternalServerError)
        }
    }
}

pub async fn decode_with_user_role(authorized_role:Vec<String>, token:&str, state:&Data<crate::AppState>) ->  Result<Claims,ApiError>{
    let (decoded,data_user) = decode_and_authenticate(token,&state).await?;

    let find_role = authorized_role.into_iter().position(|x| x==data_user.user_role);

    if data_user.user_role != "superadmin" && (data_user.status != "verified" || find_role == None) {
        return Err(ApiError::Unauthorized("Unauthorized".to_string()));
    }

    Ok(decoded)
}