use std::{collections::HashMap};

use axum::{extract::{FromRequest, Extension, RequestParts}, async_trait};
use rspotify::{AuthCodeSpotify};
use rusoto_dynamodb::{DynamoDbClient, DynamoDb, GetItemInput, AttributeValue};

use crate::errors::user_client_error::UserClientError;

pub struct UserClient(AuthCodeSpotify);

#[async_trait]
impl <B> FromRequest<B> for UserClient
where B: Send,{
type Rejection = UserClientError;
async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(dynamo_client) = Extension::<DynamoDbClient>::from_request(req).await.expect("dynamo client");
        let value = AttributeValue {
            s: Some("user_id".to_owned()),
            ..Default::default()
        };
        let key = HashMap::<String,AttributeValue>::from([("sessionid".to_string(),value)]);
        let input = GetItemInput{
            table_name :"tablename".to_string(),
            key:key,
            ..Default::default()
        };
        let output = dynamo_client.get_item(input).await?;
        let attributes = match output.item{
            Some(item) => item,
            None => {
                return Err(UserClientError::SessionParseError);
            }
        };
        let sessionid = match attributes.get("sessionid"){
            Some(sessionid) => sessionid,
            None => {
                return Err(UserClientError::SessionParseError);
            }
        };
        //トークンをダイナモから取得yo!yo!
        let spotify_client= AuthCodeSpotify::from_token(token);
        let user_client = UserClient(spotify_client);
        Ok(user_client)
    }
}