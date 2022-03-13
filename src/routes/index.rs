use axum::response::IntoResponse;

use crate::user_client::UserClient;

pub async fn get(client:UserClient)-> impl IntoResponse{
    "a".into_response()
}