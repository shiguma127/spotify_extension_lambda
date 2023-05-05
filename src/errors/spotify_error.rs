use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use rspotify::ClientError;
use thiserror::Error;
#[derive(Debug, Error)]
pub struct SpotifyClientError(ClientError);

impl From<ClientError> for SpotifyClientError {
    fn from(error: ClientError) -> Self {
        SpotifyClientError(error)
    }
}

impl Display for SpotifyClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for SpotifyClientError {
    fn into_response(self) -> axum::response::Response {
        log::error!("error: {}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response()
    }
}
