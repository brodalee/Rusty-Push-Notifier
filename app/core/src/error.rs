use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use fcm::FcmClientError;
use sqlx::{Error as DbError};
use crate::http::response::ErrorResponse;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigError(String),
    ProviderError(String),
    HeaderError(String),
    NotFoundError(String),
    MissingDataError(String),
    ValidationError(String),
    AlreadyExistError(String),
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match &value {
            Error::ConfigError(text) => text.to_owned(),
            Error::ProviderError(text) => text.to_owned(),
            Error::HeaderError(text) => text.to_owned(),
            Error::NotFoundError(text) => text.to_owned(),
            Error::MissingDataError(text) => text.to_owned(),
            Error::ValidationError(text) => text.to_owned(),
            Error::AlreadyExistError(text) => text.to_owned(),
        }
    }
}

impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        Error::ProviderError(value.to_string())
    }
}

impl From<FcmClientError> for Error {
    fn from(value: FcmClientError) -> Self {
        Error::ProviderError(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::HeaderError(_) => StatusCode::BAD_REQUEST,
            Error::NotFoundError(_) => StatusCode::NOT_FOUND,
            Error::AlreadyExistError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Error::ValidationError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text.to_string() }),
            Error::NotFoundError(text) => HttpResponse::NotFound().json(
                ErrorResponse { message: text.to_string() }
            ),
            Error::HeaderError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text.to_string() }),
            Error::MissingDataError(text) => HttpResponse::InternalServerError().json(
                ErrorResponse { message: text.to_string() }
            ),
            Error::AlreadyExistError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text.to_string() }),
            e => HttpResponse::InternalServerError().json(
                ErrorResponse { message: e.to_string() }
            )
        }
    }
}