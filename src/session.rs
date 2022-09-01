
use rspotify::Token;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub session_id: Uuid,
    pub token_json_string : String,
    pub session_expire: i64
    //pub token: Token,
}
