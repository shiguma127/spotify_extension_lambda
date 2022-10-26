use axum::{
    http::{self},
    response::{IntoResponse, Response},
    Json,
};
use rspotify::{
    clients::OAuthClient,
    model::{AdditionalType, PlayableItem},
    AuthCodeSpotify,
};


use crate::{errors::spotify_error::SpotifyError, user_client::UserClient};

pub async fn get(client: UserClient) -> Result<Response, SpotifyError> {
    let spotify: AuthCodeSpotify = client.into();
    let context = spotify
        .current_playback(
            None,
            Some(&vec![AdditionalType::Track, AdditionalType::Episode]),
        )
        .await?;

    // TODO handle no contxt
    let context = match context {
        Some(cxt) => cxt,
        None => return Ok(http::StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    let item = match context.item {
        Some(item) => item,
        None => return Ok(http::StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };
    
    match item {
        PlayableItem::Track(track) => Ok(Json(&track).into_response()),
        PlayableItem::Episode(episode) => Ok(Json(&episode).into_response()),
    }
}
