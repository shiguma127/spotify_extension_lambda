use crate::session::Session;
use crate::{auth_code::AuthCode, errors::spotify_error::SpotifyError};
use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension,
};
use chrono::Duration;
use rspotify::{clients::OAuthClient, AuthCodeSpotify};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use serde_dynamo::to_item;
pub async fn get(
    Query(query): Query<AuthCode>,
    Extension(spotify_client): Extension<AuthCodeSpotify>,
    Extension(dynamo): Extension<DynamoDbClient>,
) -> impl IntoResponse {
    let mut spotify = spotify_client.clone();
    if let Err(err) = spotify.request_token(&query.code).await {
        return SpotifyError::from(err).into_response();
    }
    //todo fix unwrap祭り
    let access_token = spotify
        .token
        .lock()
        .await
        .unwrap()
        .as_ref()
        .unwrap()
        .clone();
    let session_id = uuid::Uuid::new_v4();
    let session_expire = chrono::Utc::now() + Duration::days(30);
    let session = Session {
        session_id: session_id.clone(),
        token_json_string: serde_json::to_string(&access_token).unwrap(),
        session_expire: session_expire.timestamp(), //token: access_token,
    };
    /*
    *Scopesがスペース区切りでシリアライズされているせいでデシリアライズできない
       thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Message("invalid type: string \"user-read-playback-state user-read-currently-playing\", expected a borrowed string"))', src\user_client.rs:75:39
    * jsonではカンマ区切りのオブジェクトになっている
    * RSpotifyのカスタムシリアライザの意味ある？
    * 要検証
    */

    let item = to_item(session).unwrap();
    let input_item = PutItemInput {
        table_name: std::env::var("SESSION_TABLE").unwrap().to_string(),
        item: item,
        ..Default::default()
    };
    if let Err(err) = dynamo.put_item(input_item).await {
        return format!("{:?}", err).into_response();
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(&format!(
            "sessionid={}; SameSite=none; Path=/; secure; Max-Age={}",
            session_id,
            2592000 //30days
        ))
        .unwrap(),
    );
    (StatusCode::OK, headers, format!("OK")).into_response()
}
