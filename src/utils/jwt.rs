use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::Utc;

use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // user_id
    pub email: String,      
    pub username: String,   
    pub role: String,       // role sebagai string
    pub exp: i64,           // expiration time
    pub iat: i64,           // issued at
}

/// Buat JWT token dari user info
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
        sub: user_id.to_string(),
        email: email.to_string(),
        username: username.to_string(),
        role: format!("{:?}", role), // Convert enum ke string
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .map_err(|e| format!("Gagal membuat JWT: {}", e))
}

/// Verify dan decode JWT token

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret-key-change-in-production".to_string());
    
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation
    )
    .map(|data| data.claims)
    .map_err(|e| format!("JWT tidak valid: {}", e))
}

/// Extract token dari Authorization header (Bearer token)

pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}

/// Get user_id dari token

pub fn get_user_id_from_token(token: &str) -> Result<i64, String> {
    let claims = verify_jwt(token)?;
    claims.sub.parse::<i64>()
        .map_err(|_| "Invalid user_id in token".to_string())
}

/// Check apakah user punya role tertentu

pub fn has_role(token: &str, required_role: &str) -> Result<bool, String> {
    let claims = verify_jwt(token)?;
    Ok(claims.role == required_role)
}

/// Check apakah token sudah expired

pub fn is_token_expired(claims: &Claims) -> bool {
    let now = Utc::now().timestamp();
    now > claims.exp
}