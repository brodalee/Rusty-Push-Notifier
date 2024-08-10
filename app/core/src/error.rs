use std::fmt::{Display, Formatter};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use sqlx::{Error as DbError};
use redis::RedisError;
use fcm::FcmClientError;
use crate::responses::ErrorResponse;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigError(String),
    ProviderError(String),
    HeaderError(String),
    NotFoundError(String),
    MissingDataError(String),
    ValidationError(String),
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match &value {
            Error::ConfigError(text) => text.to_owned(),
            Error::ProviderError(text) => text.to_owned(),
            Error::HeaderError(text) => text.to_owned(),
            Error::NotFoundError(text) => text.to_owned(),
            Error::MissingDataError(text) => text.to_owned(),
            Error::ValidationError(text) => text.to_owned()
        }
    }
}

impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        Error::ProviderError(value.to_string())
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Error::ProviderError(value.to_string())
    }
}

impl From<FcmClientError> for Error {
    fn from(value: FcmClientError) -> Self {
        Error::ProviderError(value.to_string())
    }
}

impl Responder for Error {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<BoxBody> {
        match self {
            Error::ValidationError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text }),
            Error::NotFoundError(text) => HttpResponse::NotFound().json(
                ErrorResponse { message: text }
            ),
            Error::HeaderError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text }),
            Error::MissingDataError(text) => HttpResponse::InternalServerError().json(
                ErrorResponse { message: text }
            ),
            _ => HttpResponse::InternalServerError().json(
                ErrorResponse { message: "Internal Server Error".to_string() }
            )
        }
    }
}

impl From<Error> for HttpResponse {
    fn from(value: Error) -> Self {
        match value {
            Error::ValidationError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text }),
            Error::NotFoundError(text) => HttpResponse::NotFound().json(
                ErrorResponse { message: text }
            ),
            Error::HeaderError(text) => HttpResponse::BadRequest().json(ErrorResponse { message: text }),
            Error::MissingDataError(text) => HttpResponse::InternalServerError().json(
                ErrorResponse { message: text }
            ),
            _ => HttpResponse::InternalServerError().json(
                ErrorResponse { message: "Internal Server Error".to_string() }
            )
        }
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
            _ => HttpResponse::InternalServerError().json(
                ErrorResponse { message: "Internal Server Error".to_string() }
            )
        }
    }
}