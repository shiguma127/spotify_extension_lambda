use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    headers::Cookie,
    TypedHeader,
};

use rspotify::{AuthCodeSpotify, Token};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde_dynamo::from_item;

use crate::{
    errors::user_client_error::{SessionParseError, SessionParseErrorReason, UserClientError},
    session::Session,
};

use log::info;

pub struct UserClient(AuthCodeSpotify);

impl From<AuthCodeSpotify> for UserClient {
    fn from(auth_code_spotify: AuthCodeSpotify) -> Self {
        UserClient(auth_code_spotify)
    }
}

impl Into<AuthCodeSpotify> for UserClient {
    fn into(self) -> AuthCodeSpotify {
        self.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for UserClient
where
    B: Send,
{
    type Rejection = UserClientError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = TypedHeader::<Cookie>::from_request(req).await;
        let cookie = match cookie {
            Ok(cookie) => cookie,
            Err(_) => {
                info!("{}\nTypedHeader'{:?}", "cant get cookie", cookie);
                return Err(UserClientError::SessionParseError(SessionParseError::new(
                    SessionParseErrorReason::CookieisMissing,
                )));
            }
        };
        let session_id = match cookie.get("sessionid") {
            Some(session_id) => session_id.to_string(),
            None => {
                info!(
                    "{}\ncookie:{:?}",
                    "request header has not have sessionid", cookie
                );
                return Err(UserClientError::SessionParseError(SessionParseError::new(
                    SessionParseErrorReason::SessionisMissing,
                )));
            }
        };
        let Extension(dynamo_client) = Extension::<DynamoDbClient>::from_request(req)
            .await
            .expect("dynamo client");

        let value = AttributeValue {
            s: Some(session_id),
            ..Default::default()
        };

        let key = HashMap::<String, AttributeValue>::from([("session_id".to_string(), value)]);
        let table_name = match std::env::var("SESSION_TABLE") {
            Ok(table_name) => table_name,
            Err(_) => {
                return Err(UserClientError::TableMissingError(String::from(
                    "can not get tablename from env. must set table name to SESSION_TABLE",
                )))
            }
        };
        let input = GetItemInput {
            table_name: table_name,
            key,
            ..Default::default()
        };

        let output = dynamo_client.get_item(input).await?;
        let item = match output.item {
            Some(item) => item,
            None => {
                return Err(UserClientError::SessionParseError(SessionParseError::new(
                    SessionParseErrorReason::CookieisMissing,
                )))
            }
        };

        let session = match from_item(item) {
            Ok(session) => session,
            Err(_) => {
                return Err(UserClientError::SessionParseError(SessionParseError::new(
                    SessionParseErrorReason::CanNotParseDinamoItem,
                )))
            }
        };
        let session: Session = session;
        let token: Token = serde_json::from_str(&session.token_json_string).unwrap();
        let spotify_client = AuthCodeSpotify::from_token(token);
        let user_client = UserClient(spotify_client);
        info!("{}", "restored user_client successflly.");
        Ok(user_client)
    }
}
