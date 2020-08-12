use actix_web::{
    HttpResponse,
    error::{ResponseError},
    http
};
use derive_more::Display;
use serde::{Serialize,Deserialize};
use std::{
    convert::From,
    env::VarError,
};
use sqlx::error::Error as SqlxError;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use lettre::error::Error as lettre_error;
use lettre::transport::smtp::error::Error as lettre_smtp_error;

#[derive(Debug, Display, PartialEq)]
pub enum ApiError{
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    InternalServerError,
    InternalServerErrorWithMessage(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    msg: T,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse{msg}),
            ApiError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse{msg}),
            ApiError::Forbidden(msg) => HttpResponse::Forbidden().json(ErrorResponse{msg}),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse{msg}),
            ApiError::Conflict(msg) => HttpResponse::Conflict().json(ErrorResponse{msg}),
            ApiError::InternalServerErrorWithMessage(msg) => HttpResponse::InternalServerError().json(ErrorResponse{msg}),
            ApiError::InternalServerError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

impl From<VarError> for ApiError {
    fn from(error: VarError) -> Self {
        println!("{}",error.to_string());
        ApiError::InternalServerError
    }
}

impl From<http::header::InvalidHeaderValue> for ApiError {
    fn from(error: http::header::InvalidHeaderValue) -> Self {
        println!("{}",error.to_string());
        ApiError::InternalServerError
    }
}

impl From<JwtError> for ApiError {
    fn from(error: JwtError) -> Self {
        println!("{}",error.to_string());
        ApiError::InternalServerError
    }
}

impl From<SqlxError> for ApiError {
    fn from(error: SqlxError) -> Self {
        ApiError::InternalServerErrorWithMessage(error.to_string())
    }
}

impl From<BcryptError> for ApiError {
    fn from(error: BcryptError) -> Self {
        ApiError::InternalServerErrorWithMessage(error.to_string())
    }
}

impl From<lettre_error> for ApiError {
    fn from(error: lettre_error) -> Self {
        println!("{}",error.to_string());
        ApiError::InternalServerError
    }
}

impl From<lettre_smtp_error> for ApiError {
    fn from(error: lettre_smtp_error) -> Self {
        println!("{}",error.to_string());
        ApiError::InternalServerError
    }
}