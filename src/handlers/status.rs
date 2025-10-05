use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::services::payment_service::PaymentService;

pub async fn xendit_status(
    Extension(payment_service): Extension<Arc<PaymentService>>,
) -> impl IntoResponse {
    let config = payment_service.get_config();
    
    let is_active = !config.secret_key.is_empty() 
        && !config.webhook_token.is_empty()
        && config.validate().is_ok();
    
    let mode = if config.is_production {
        "production"
    } else {
        "development"
    };

    (
        StatusCode::OK,
        Json(json!({
            "service": "xendit",
            "status": if is_active { "active" } else { "inactive" },
            "mode": mode,
            "api_url": config.api_url,
            "configured": true
        })),
    )
}

pub async fn webhook_status() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "service": "webhook",
            "status": "ready",
            "endpoint": "/webhook/xendit",
            "methods": ["POST"],
            "ready": true
        })),
    )
}

pub async fn system_status(
    Extension(payment_service): Extension<Arc<PaymentService>>,
) -> impl IntoResponse {
    let config = payment_service.get_config();
    
    let xendit_active = !config.secret_key.is_empty() 
        && !config.webhook_token.is_empty()
        && config.validate().is_ok();

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "services": {
                "xendit": {
                    "status": if xendit_active { "active" } else { "inactive" },
                    "mode": config.environment()
                },
                "webhook": {
                    "status": "ready",
                    "endpoint": "/webhook/xendit"
                },
                "database": {
                    "status": "connected"
                }
            }
        })),
    )
}