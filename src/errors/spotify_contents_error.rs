use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub enum SpotifyContentError {
    NoplayableContent(NoPlayableContentError),
}

impl IntoResponse for SpotifyContentError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SpotifyContentError::NoplayableContent(err) => err.into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub struct NoPlayableContentError;

impl std::fmt::Display for NoPlayableContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No Playable Content")
    }
}
impl IntoResponse for NoPlayableContentError {
    fn into_response(self) -> axum::response::Response {
        log::error!("error: {}", self);
        (StatusCode::NOT_FOUND, "No Playable Content").into_response()
    }
}
