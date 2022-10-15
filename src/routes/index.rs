use axum::{
    http::{self},
    response::IntoResponse,
    Json,
};
use rspotify::{
    clients::OAuthClient,
    model::{AdditionalType, PlayableItem},
    AuthCodeSpotify,
};

use crate::{errors::spotify_error::SpotifyError, user_client::UserClient};

pub async fn get(client: UserClient) -> impl IntoResponse {
    let spotify: AuthCodeSpotify = client.into();
    let result = spotify
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

    // TODO handle no contxt
    let context = match context {
        Some(cxt) => cxt,
        None => return http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let item = match context.item {
        Some(item) => item,
        None => return http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match item {
        PlayableItem::Track(track) => Json(&track).into_response(),
        PlayableItem::Episode(episode) => Json(&episode).into_response(),
    }
}
