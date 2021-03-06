use axum::{extract::Extension, routing::get, Router};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use rusoto_dynamodb::DynamoDbClient;
use std::{net::SocketAddr};
use dotenv::dotenv;
mod app_error;
mod errors;
mod routes;
mod user_client;
mod session;
mod auth_code;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    dotenv().ok();
    let credentials = Credentials {
        id: std::env::var("CLIENT_ID")?,
        secret: Some(std::env::var("CLIENT_SECRET")?),
    };
    let dynamo = DynamoDbClient::new(rusoto_core::Region::ApNortheast1);
    let oauth = OAuth {
        redirect_uri: std::env::var("CALLBACK_URI")?,
        scopes: scopes!("user-read-currently-playing", "user-read-playback-state"),
        ..Default::default()
    };
    
    let spotify_client = AuthCodeSpotify::new(credentials, oauth);
    let app = Router::new()
        .route("/", get(routes::index::get))
        .route("/login", get(routes::login::get))
        .route("/callback", get(routes::callback::get))
        .layer(Extension(spotify_client))
        .layer(Extension(dynamo));
    if is_running_on_lambda() {
        // Run app on AWS Lambda
        run_hyper_on_lambda(app).await?;
    } else {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}
