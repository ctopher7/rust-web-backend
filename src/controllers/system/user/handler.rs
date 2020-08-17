use actix_web::{
    web::{Query,Data},
    HttpResponse,
    http::{header::SET_COOKIE,HeaderValue}
};
use sqlx::query;

use crate::utils::{
    error::ApiError,
    auth::{decode_and_authenticate,create_auth_cookie},
};

pub async fn verify_email(
    req_query:Query<super::data::UserVerificationQuery>,
    state:Data<crate::AppState>
) -> Result<HttpResponse,ApiError>{
    let (decoded,_) = decode_and_authenticate(req_query.token.as_str(), &state).await?;

    query!(
        "UPDATE users SET user_status_id = (
            SELECT id FROM user_status WHERE status = 'verified'
        ) WHERE id = $1;",
        decoded.id
    ).execute(&state.db_postgres).await?;

    let auth_cookie = create_auth_cookie(decoded.id, &state).await;
    
    Ok(HttpResponse::Ok().set_header(
        SET_COOKIE, 
        HeaderValue::from_str(&auth_cookie)?
    ).finish())
}