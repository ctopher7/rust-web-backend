use actix_web::{
    web::{Json,Query,Data}
};
use sqlx::{query_as,query};
use bcrypt::{hash};

use crate::{
    utils::{
        error::ApiError,
        http_body::{MessageWithData,Message},
        paginate,
        validator::validate_input
    }
};

use super::data::{User,CreateUser};

pub async fn all(
    query:Query<paginate::QueryPagination>,
    state:Data<crate::AppState>,
)->Result<Json<MessageWithData<Vec<User>>>,ApiError>{
    let data = query_as!(User,
    "SELECT * FROM users ORDER BY $1 OFFSET $2 LIMIT $3;",
        format!("{} {}","created_at",query.get_order()),
        query.get_offset(),
        query.get_limit()
    ).fetch_all(&state.db_postgres).await?;

    Ok(Json(MessageWithData{
        msg: "ok",
        data
    }))
}

pub async fn create(
    body:Json<CreateUser>,
    state:Data<crate::AppState>
)->Result<Json<Message>,ApiError>{
    validate_input(&body)?;

    let dob_splitted:Vec<u32> = body.date_of_birth.clone().unwrap().split("-").into_iter().map(|x|->u32{
        x.parse::<u32>().unwrap()
    }).collect();

    query!(
        "INSERT INTO users(
            user_role_id,
            user_status_id,
            email,
            phone_number,
            password,
            name,
            date_of_birth,
            last_logged_in
        )
        VALUES (
            (SELECT id FROM user_roles WHERE role = $1),
            (SELECT id FROM user_status WHERE status = 'verified'),
            $2,$3,$4,$5,$6,$7
        );
        ",
        &body.user_role,
        &body.email,
        &body.phone_number,
        hash(&body.password,6)?,
        body.name.clone().unwrap(),
        chrono::naive::NaiveDate::from_ymd(dob_splitted[0] as i32,dob_splitted[1],dob_splitted[2]),
        chrono::Utc::now()
    ).execute(&state.db_postgres).await?;

    Ok(Json(Message{
        msg:"OK"
    }))
}