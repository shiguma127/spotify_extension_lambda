use axum::{
    response::{IntoResponse, Redirect, Result},
    Extension,
};
use rspotify::AuthCodeSpotify;

use crate::errors::spotify_error::SpotifyError;

pub async fn get(
    Extension(spotify_client): Extension<AuthCodeSpotify>,
) -> Result<impl IntoResponse> {
    let url = spotify_client
        .get_authorize_url(false)
        .map_err(SpotifyError::from)?;
    Ok(Redirect::permanent(url.as_str()))
}
