use axum::{http, response::IntoResponse};
use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;

pub enum AppError {
    RusotoError(RusotoError<GetItemError>),
}

impl From<RusotoError<GetItemError>> for AppError {
    fn from(err: RusotoError<GetItemError>) -> Self {
        AppError::RusotoError(err)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::RusotoError(_) => http::StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
