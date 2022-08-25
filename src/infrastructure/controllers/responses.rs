use crate::domain::UserDomainError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NoContentResponse {}

impl ResponseError for UserDomainError {
    fn status_code(&self) -> StatusCode {
        return match &self {
            UserDomainError::InvalidUuidUserId => StatusCode::BAD_REQUEST,
            UserDomainError::InvalidUserEmail(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::CouldNotGeneratePassword(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::ProblemRetrievingPassword(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::UserAlreadyRegistered(_string) => StatusCode::BAD_REQUEST,
            UserDomainError::CommandNotYetImplemented(_string) => StatusCode::INTERNAL_SERVER_ERROR,
            UserDomainError::CouldNotLoadUserEvents(_string) => StatusCode::INTERNAL_SERVER_ERROR,
            UserDomainError::CouldNotSaveUserEvents(_string) => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
