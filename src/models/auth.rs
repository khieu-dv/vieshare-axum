use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub access_token: String,
    pub refresh_token: String,
    pub access_uuid: String,
    pub refresh_uuid: String,
    pub at_expires: i64,
    pub rt_expires: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessDetails {
    pub access_uuid: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub authorized: bool,
    pub access_uuid: Option<String>,
    pub refresh_uuid: Option<String>,
    pub user_id: String,
    pub exp: i64,
}

pub struct AuthModel;

impl AuthModel {
    pub fn new() -> Self {
        Self
    }

    pub fn create_token(&self, user_id: &str) -> Result<TokenDetails, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let at_expires = (now + Duration::minutes(15)).timestamp();
        let rt_expires = (now + Duration::days(7)).timestamp();
        
        let access_uuid = Uuid::new_v4().to_string();
        let refresh_uuid = Uuid::new_v4().to_string();

        let access_secret = std::env::var("ACCESS_SECRET").unwrap_or_else(|_| "access-secret".to_string());
        let refresh_secret = std::env::var("REFRESH_SECRET").unwrap_or_else(|_| "refresh-secret".to_string());

        // Creating Access Token
        let at_claims = Claims {
            authorized: true,
            access_uuid: Some(access_uuid.clone()),
            refresh_uuid: None,
            user_id: user_id.to_string(),
            exp: at_expires,
        };

        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &at_claims,
            &EncodingKey::from_secret(access_secret.as_ref()),
        )?;

        // Creating Refresh Token
        let rt_claims = Claims {
            authorized: false,
            access_uuid: None,
            refresh_uuid: Some(refresh_uuid.clone()),
            user_id: user_id.to_string(),
            exp: rt_expires,
        };

        let refresh_token = encode(
            &Header::new(Algorithm::HS256),
            &rt_claims,
            &EncodingKey::from_secret(refresh_secret.as_ref()),
        )?;

        Ok(TokenDetails {
            access_token,
            refresh_token,
            access_uuid,
            refresh_uuid,
            at_expires,
            rt_expires,
        })
    }

    pub fn extract_token(&self, auth_header: Option<&str>) -> Option<String> {
        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                return Some(header[7..].to_string());
            }
        }
        None
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let access_secret = std::env::var("ACCESS_SECRET").unwrap_or_else(|_| "access-secret".to_string());
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(access_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims)
    }

    pub fn extract_token_metadata(&self, auth_header: Option<&str>) -> Result<AccessDetails, Box<dyn std::error::Error>> {
        if let Some(token) = self.extract_token(auth_header) {
            let claims = self.verify_token(&token)?;
            
            if let Some(access_uuid) = claims.access_uuid {
                return Ok(AccessDetails {
                    access_uuid,
                    user_id: claims.user_id,
                });
            }
        }
        
        Err("Invalid token".into())
    }
}