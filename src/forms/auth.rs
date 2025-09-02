use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Token {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}