use axum::{http, response::IntoResponse};
use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum UserClientError {
    #[error("cannot get session from dynamo")]
    RusotoError(#[from] RusotoError<GetItemError>),
    #[error("cannot parse session from cookie")]
    SessionParseError(SessionParseError),
}

#[derive(Debug)]
pub struct SessionParseError {
    reson: SessionParseErrorReason,
}

#[derive(Debug)]
pub enum SessionParseErrorReason {
    CookieisMissing,
    SessionisMissing,
}

impl SessionParseError {
    pub fn new(reason: SessionParseErrorReason) -> Self {
        SessionParseError { reson: reason }
    }
}

impl IntoResponse for UserClientError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UserClientError::RusotoError(_) | UserClientError::SessionParseError(_) => {
                (http::StatusCode::UNAUTHORIZED, "Unauthrised").into_response()
            }
        }
    }
}
