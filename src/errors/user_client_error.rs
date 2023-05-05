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
    #[error("cannot parse session from cookie")]
    TableMissingError(String),
}

#[derive(Debug)]
pub struct SessionParseError {
    reson: SessionParseErrorReason,
}

#[derive(Debug)]
pub enum SessionParseErrorReason {
    CookieisMissing,
    SessionisMissing,
    CanNotParseDinamoItem,
}

impl SessionParseError {
    pub fn new(reason: SessionParseErrorReason) -> Self {
        SessionParseError { reson: reason }
    }
}

impl IntoResponse for UserClientError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UserClientError::RusotoError(e) => {
                log::error!("{:?}, rusoto error", e);
                (http::StatusCode::UNAUTHORIZED, "Unauthrised").into_response()
            }
            UserClientError::SessionParseError(e) => {
                log::info!("Failed to parse session. reason: {:?}", e.reson);
                (http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response()
            }
            UserClientError::TableMissingError(e) => {
                log::error!("Faild to parse session. reason ({:?})", e);
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                )
                    .into_response()
            }
        }
    }
}
