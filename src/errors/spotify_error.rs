use std::fmt::Display;

use axum::response::IntoResponse;
use rspotify::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct SpotifyError(ClientError);

impl From<ClientError> for SpotifyError {
    fn from(error: ClientError) -> Self {
        SpotifyError(error)
    }
}

impl Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for SpotifyError {
    fn into_response(self) -> axum::response::Response {
        format!("{}", self).into_response()
    }
}
