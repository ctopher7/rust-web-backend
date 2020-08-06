use actix_web::web::{Query,Data,Json};
use sqlx::query;

use crate::utils::{
    error::ApiError,
    auth::decode_and_authenticate,
    http_body::Message
};

pub async fn verify_email(
    req_query:Query<super::data::UserVerificationQuery>,
    state:Data<crate::AppState>
) -> Result<Json<Message>,ApiError>{
    let (decoded,_) = decode_and_authenticate(req_query.token.as_str(), &state).await?;

    let data_user_status = query!(
        "SELECT * FROM user_status WHERE status = 'verified';"
    ).fetch_one(&state.db_postgres).await?;

    query!(
        r#"UPDATE users SET user_status_id = $1 WHERE id = $2;"#,
        data_user_status.id, decoded.id
    ).execute(&state.db_postgres).await?;

    Ok(Json(Message{
        msg:"ok"
    }))
}