use axum::{extract::Extension, middleware, routing::get, Router};
use dotenv::dotenv;
use env_logger;
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use middlewares::{cors_layer, logging_request};
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use rusoto_dynamodb::DynamoDbClient;
use std::net::SocketAddr;

mod auth_code;
mod errors;
mod middlewares;
mod routes;
mod session;
mod user_client;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    dotenv().ok();
    env_logger::init();
    let dynamo = DynamoDbClient::new(rusoto_core::Region::ApNortheast1);
    let credentials = Credentials {
        id: std::env::var("CLIENT_ID")?,
        secret: Some(std::env::var("CLIENT_SECRET")?),
    };
    let config = Config {
        token_refreshing: true,
        ..Default::default()
    };
    let oauth = OAuth {
        redirect_uri: std::env::var("CALLBACK_URI")?,
        scopes: scopes!("user-read-currently-playing", "user-read-playback-state"),
        ..Default::default()
    };
    let spotify_client = AuthCodeSpotify::with_config(credentials, oauth, config.clone());
    let app = Router::new()
        .route("/", get(routes::index::get))
        .route("/login", get(routes::login::get))
        .route("/callback", get(routes::callback::get))
        .layer(Extension(spotify_client))
        .layer(Extension(dynamo))
        .layer(middleware::from_fn(logging_request::logging))
        .layer(cors_layer::cors());
    if is_running_on_lambda() {
        // Run app on AWS Lambda
        run_hyper_on_lambda(app).await?;
    } else {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        println!("running on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}
