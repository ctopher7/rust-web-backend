use sqlx::{PgPool};
use std::env::var;
use crate::utils::error::ApiError;

pub async fn create_connection()->Result<PgPool,ApiError>{
    Ok(PgPool::new(&var("DATABASE_URL")?).await?)
}