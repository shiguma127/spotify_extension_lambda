use axum::{response::IntoResponse, http};
use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum UserClientError {
    #[error("cannot get session from dynamo")]
    RusotoError(#[from] RusotoError<GetItemError>),
    #[error("CannotParseSession")]
    SessionParseError, 
}

impl IntoResponse for UserClientError{
    fn into_response(self) -> axum::response::Response {
        match self {
            UserClientError::RusotoError(_) | 
            UserClientError::SessionParseError  => http::StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}