use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuthCode {
    pub code: String,
}
