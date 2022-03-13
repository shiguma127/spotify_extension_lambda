use axum::{routing::get, Router,extract::Extension,};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use rspotify::{Credentials, OAuth, scopes, Config, AuthCodeSpotify};
use rusoto_dynamodb::DynamoDbClient;
use std::{net::SocketAddr, sync::Arc};
mod routes;
mod user_client;
mod app_error;
mod errors;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let credentials = Credentials{
        id: std::env::var("client_id")?,
        secret: Some(std::env::var("client_secret")?),
    };
    let dynamo = DynamoDbClient::new(Default::default());
    let oauth = OAuth {
        redirect_uri: std::env::var("callback_url")?,
        scopes: scopes!("user-read-currently-playing", "user-read-playback-state"),
        ..Default::default()
    };
    let spotify_client = AuthCodeSpotify::new(credentials, oauth);
    let app = Router::new()
        .route("/", get(routes::index::get))
        .route("/login", get(root))
        .route("/callback", get(root))
        .layer(Extension(spotify_client))
        .layer(Extension(dynamo));
    if is_running_on_lambda() {
        // Run app on AWS Lambda
        run_hyper_on_lambda(app).await?;
    } else {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    }
    Ok(())
}