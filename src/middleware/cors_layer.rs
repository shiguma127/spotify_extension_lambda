use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            "https://tweetdeck.twitter.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET])
        .allow_credentials(true)
}
