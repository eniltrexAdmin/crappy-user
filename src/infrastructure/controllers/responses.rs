use std::fmt::{Display, Formatter};
use crate::domain::{UserDomainError, UserViewRepositoryError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::JsonPayloadError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NoContentResponse {}

#[derive(Debug, Serialize)]
pub struct SuccessfulAuthenticationResponse {
    pub(crate) token: String
}

impl ResponseError for UserDomainError {
    fn status_code(&self) -> StatusCode {
        return match &self {
            UserDomainError::InvalidUuidUserId => StatusCode::BAD_REQUEST,
            UserDomainError::InvalidUserEmail(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::CouldNotGeneratePassword(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::ProblemRetrievingPassword(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::UserAlreadyRegistered(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::IncorrectPassword => StatusCode::BAD_REQUEST,
            UserDomainError::UserNotFound(_string) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl ResponseError for UserViewRepositoryError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct CrappyActixError {
    pub error_message: String
}

impl Display for CrappyActixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actix error: {}", self.error_message)
    }
}

impl ResponseError for CrappyActixError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(self)
    }
}

impl From<JsonPayloadError> for CrappyActixError {
    fn from(err: JsonPayloadError) -> Self {
        CrappyActixError{
            error_message: err.to_string()
        }
    }
}
