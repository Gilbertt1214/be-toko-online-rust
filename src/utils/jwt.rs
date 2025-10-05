use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub email: String,
    pub username: String,
    pub role: UserRole,
    pub exp: i64,
    pub iat: i64,
}

/// Create JWT token from user info
pub fn create_jwt(
    user_id: i64,
    email: &str,
    username: &str,
    role: UserRole,
) -> Result<String, String> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret-key-change-in-production".to_string());

    let now = Utc::now();
    let expiration = now + chrono::Duration::hours(24);

    let claims = Claims {
        user_id,
        email: email.to_string(),
        username: username.to_string(),
        role,
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Failed to create JWT: {}", e))
}

/// Verify and decode JWT token
pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret-key-change-in-production".to_string());

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid JWT: {}", e))
}