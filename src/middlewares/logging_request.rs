use axum::{
    http::{Request, Response},
    middleware::Next,
    response::IntoResponse,
};
use log::info;
pub async fn logging<B>(req: Request<B>, next: Next<B>) -> Result<impl IntoResponse, Response<B>> {
    info!("{:?} {:?}", req.method(), req.uri());
    info!("{:?}", req.headers());
    Ok(next.run(req).await)
}
