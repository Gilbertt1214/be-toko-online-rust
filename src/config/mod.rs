pub mod xendit;
pub mod app;

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            secret_key: env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
        }
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate database URL
        if !self.database_url.starts_with("postgresql://") {
            return Err("DATABASE_URL must start with postgresql://".into());
        }

        // Validate keys length
        if self.secret_key.len() < 32 {
            return Err("SECRET_KEY too short (minimum 32 characters)".into());
        }

        if self.jwt_secret.len() < 32 {
            return Err("JWT_SECRET too short (minimum 32 characters)".into());
        }

        Ok(())
    }
}