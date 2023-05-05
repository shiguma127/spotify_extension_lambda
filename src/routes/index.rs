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

use crate::{
    errors::spotify_contents_error::NoPlayableContentError,
    errors::{
        app_error::AppError, spotify_contents_error::SpotifyContentError,
        spotify_error::SpotifyClientError,
    },
    user_client::UserClient,
};

// TODO error をすべて app errorに集約する。
pub async fn get(client: UserClient) -> Result<Response, AppError> {
    let spotify: AuthCodeSpotify = client.into();
    let context = match spotify
        .current_playback(
            None,
            Some(&vec![AdditionalType::Track, AdditionalType::Episode]),
        )
        .await
    {
        Ok(context) => context,
        Err(err) => return Err(AppError::SpotifyClient(SpotifyClientError::from(err))),
    };

    // TODO handle no contxt
    let context = match context {
        Some(cxt) => cxt,
        None => {
            return Err(AppError::SpotifyContents(
                SpotifyContentError::NoplayableContent(NoPlayableContentError),
            ))
        }
    };

    let item = match context.item {
        Some(item) => item,
        None => {
            return Err(AppError::SpotifyContents(
                SpotifyContentError::NoplayableContent(NoPlayableContentError),
            ))
        }
    };

    match item {
        PlayableItem::Track(track) => Ok(Json(&track).into_response()),
        PlayableItem::Episode(episode) => Ok(Json(&episode).into_response()),
    }
}
