use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use std::env;

// Compatible with base64 v0.22
#[derive(Debug, Clone, Deserialize)]
pub struct XenditConfig {
    pub secret_key: String,
    pub public_key: String,
    pub webhook_token: String,
    pub is_production: bool,
    pub api_url: String,
    pub success_redirect_url: String,
    pub failure_redirect_url: String,
}

impl XenditConfig {
    /// Load konfigurasi dari environment variables
    pub fn from_env() -> Self {
        Self {
            secret_key: env::var("XENDIT_SECRET_KEY")
                .expect("XENDIT_SECRET_KEY must be set"),
            public_key: env::var("XENDIT_PUBLIC_KEY")
                .expect("XENDIT_PUBLIC_KEY must be set"),
            webhook_token: env::var("XENDIT_WEBHOOK_TOKEN")
                .expect("XENDIT_WEBHOOK_TOKEN must be set"),
            is_production: env::var("XENDIT_IS_PRODUCTION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            api_url: env::var("XENDIT_API_URL")
                .unwrap_or_else(|_| "https://api.xendit.co".to_string()),
            success_redirect_url: env::var("XENDIT_SUCCESS_REDIRECT_URL")
                .expect("XENDIT_SUCCESS_REDIRECT_URL must be set"),
            failure_redirect_url: env::var("XENDIT_FAILURE_REDIRECT_URL")
                .expect("XENDIT_FAILURE_REDIRECT_URL must be set"),
        }
    }

    /// Generate Basic Auth header untuk Xendit API
    pub fn get_basic_auth(&self) -> String {
        let credentials = format!("{}:", self.secret_key);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        format!("Basic {}", encoded)
    }

    /// Get API key untuk beberapa endpoint yang memerlukan format berbeda
    // pub fn get_api_key(&self) -> &str {
    //     &self.secret_key
    // }

    /// Validate konfigurasi Xendit
    pub fn validate(&self) -> Result<(), String> {
        // Check secret key format
        if self.is_production {
            if !self.secret_key.starts_with("xnd_production_") {
                return Err(
                    "Production mode requires production secret key (xnd_production_*)".to_string()
                );
            }
        } else {
            if !self.secret_key.starts_with("xnd_development_") {
                return Err(
                    "Development mode requires development secret key (xnd_development_*)"
                        .to_string(),
                );
            }
        }

        // Check public key format
        if self.is_production {
            if !self.public_key.starts_with("xnd_public_production_") {
                return Err("Production mode requires production public key".to_string());
            }
        } else {
            if !self.public_key.starts_with("xnd_public_development_") {
                return Err("Development mode requires development public key".to_string());
            }
        }

        // Check webhook token
        if self.webhook_token.is_empty() {
            return Err("Webhook token cannot be empty".to_string());
        }

        if self.webhook_token.len() < 32 {
            return Err("Webhook token too short (minimum 32 characters)".to_string());
        }

        // Check URLs
        if !self.success_redirect_url.starts_with("http") {
            return Err("Invalid success redirect URL (must start with http/https)".to_string());
        }

        if !self.failure_redirect_url.starts_with("http") {
            return Err("Invalid failure redirect URL (must start with http/https)".to_string());
        }

        // Check API URL
        if !self.api_url.starts_with("https://") {
            return Err("API URL must use HTTPS".to_string());
        }

        Ok(())
    }

    /// Get environment mode string untuk logging
    pub fn environment(&self) -> &str {
        if self.is_production {
            "Production"
        } else {
            "Development"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_generation() {
        let config = XenditConfig {
            secret_key: "xnd_development_test123".to_string(),
            public_key: "xnd_public_development_test".to_string(),
            webhook_token: "test_webhook_token_with_minimum_length".to_string(),
            is_production: false,
            api_url: "https://api.xendit.co".to_string(),
            success_redirect_url: "http://localhost:8000/success".to_string(),
            failure_redirect_url: "http://localhost:8000/failed".to_string(),
        };

        let auth = config.get_basic_auth();
        assert!(auth.starts_with("Basic "));
    }

    #[test]
    fn test_environment() {
        let mut config = XenditConfig {
            secret_key: "xnd_development_test".to_string(),
            public_key: "xnd_public_development_test".to_string(),
            webhook_token: "test_webhook_token_with_minimum_length".to_string(),
            is_production: false,
            api_url: "https://api.xendit.co".to_string(),
            success_redirect_url: "http://localhost:8000/success".to_string(),
            failure_redirect_url: "http://localhost:8000/failed".to_string(),
        };

        assert_eq!(config.environment(), "Development");

        config.is_production = true;
        assert_eq!(config.environment(), "Production");
    }
}