use axum::{response::IntoResponse, http::{self}, Json};
use rspotify::{AuthCodeSpotify, clients::OAuthClient, model::{AdditionalType}};

use crate::{user_client::UserClient, errors::spotify_error::SpotifyError};

pub async fn get(client: UserClient) -> impl IntoResponse {
    let spotify :AuthCodeSpotify = client.into();
    let result  = spotify
        .current_playback(
            None,
            Some(&vec![AdditionalType::Track, AdditionalType::Episode]),
        )
        .await;
    let context = match result {
        Ok(context) => context,
        Err(error) => {
            return SpotifyError::from(error).into_response();
        }
    };
    match context {
        Some(cxt)=> Json(cxt).into_response(),
        None => http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
