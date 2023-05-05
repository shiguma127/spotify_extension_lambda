use axum::{http, response::IntoResponse};

use super::{
    spotify_contents_error::SpotifyContentError, spotify_error::SpotifyClientError,
    user_client_error::UserClientError,
};

pub enum AppError {
    SpotifyContents(SpotifyContentError),
    SpotifyClient(SpotifyClientError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::SpotifyContents(err) => err.into_response(),
            AppError::SpotifyClient(err) => err.into_response(),
        }
    }
}
