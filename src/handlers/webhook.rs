use axum::{
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::models::{order, payment, prelude::*};
use crate::services::payment_service::{PaymentService, PaymentStatus, XenditWebhookPayload};

// ==================== POST HANDLER (Webhook dari Xendit) ====================

pub async fn xendit_webhook(
    Extension(payment_service): Extension<Arc<PaymentService>>,
    Extension(db): Extension<DatabaseConnection>,
    headers: HeaderMap,
    Json(payload): Json<XenditWebhookPayload>,
) -> Response {
    info!("========================================");
    info!("Received Xendit webhook notification");
    info!("========================================");
    info!("  Invoice ID: {}", payload.id);
    info!("  External ID: {}", payload.external_id);
    info!("  Status: {}", payload.status);
    info!("  Amount: {}", payload.amount);
    info!("  Paid Amount: {}", payload.paid_amount);
    info!("  Payment Channel: {}", payload.payment_channel);
    info!("  Payment Method: {}", payload.payment_method);
    info!("========================================");

    let callback_token = headers
        .get("x-callback-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !payment_service.verify_webhook_token(callback_token) {
        warn!("UNAUTHORIZED: Invalid webhook token");
        return error_response(StatusCode::UNAUTHORIZED, "Invalid webhook token");
    }

    info!("Token verification: SUCCESS");

    let order_id = match extract_order_id(&payload.external_id) {
        Ok(id) => {
            info!("Extracted Order ID: {}", id);
            id
        }
        Err(e) => {
            error!("BAD REQUEST: {}", e);
            return error_response(StatusCode::BAD_REQUEST, &e);
        }
    };

    let payment_status = payment_service.process_webhook(&payload);
    info!("Payment status processed: {:?}", payment_status);

    match process_payment_webhook(&db, order_id, &payment_status, &payload).await {
        Ok((payment_record, order_record)) => {
            info!("DATABASE UPDATE: SUCCESS");
            info!("  Payment ID: {}", payment_record.id);
            info!("  Order ID: {}", order_record.id);
            info!("  Payment Status: {}", payment_record.status);
            info!("  Order Status: {}", order_record.status);

            if let Err(e) = handle_post_payment_actions(&payment_status, order_id, &payload).await {
                warn!("Post-payment actions warning: {}", e);
            }

            info!("========================================");
            info!("Webhook processing completed");
            info!("========================================");

            success_response(payment_record.id, &payment_record.status, &order_record.status)
        }
        Err(e) => {
            error!("DATABASE UPDATE: FAILED");
            error!("  Error: {}", e);
            error!("========================================");
            error_response(StatusCode::INTERNAL_SERVER_ERROR, &e)
        }
    }
}

// ==================== GET HANDLER (Info Page) ====================

/// GET handler untuk webhook endpoint - menampilkan dokumentasi
pub async fn webhook_info() -> impl IntoResponse {
    Html(include_str!("../templates/webhook_info.html"))
}

// ==================== HELPER FUNCTIONS ====================

fn extract_order_id(external_id: &str) -> Result<i64, String> {
    external_id
        .strip_prefix("ORDER-")
        .ok_or_else(|| format!("External ID must start with 'ORDER-', got: {}", external_id))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid order ID in: {}", external_id))
}

async fn process_payment_webhook(
    db: &DatabaseConnection,
    order_id: i64,
    payment_status: &PaymentStatus,
    payload: &XenditWebhookPayload,
) -> Result<(payment::Model, order::Model), String> {
    info!("Starting database transaction");

    let txn = db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    let order = Order::find_by_id(order_id)
        .one(&txn)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| format!("Order {} not found", order_id))?;

    info!("Found order #{} with status: {}", order.id, order.status);

    let existing_payment = payment::Entity::find()
        .filter(payment::Column::ExternalId.eq(&payload.external_id))
        .one(&txn)
        .await
        .map_err(|e| format!("Failed to query payment: {}", e))?;

    let payment = if let Some(existing) = existing_payment {
        info!("Updating existing payment record #{}", existing.id);

        let mut payment_active: payment::ActiveModel = existing.into();
        payment_active.invoice_id = Set(Some(payload.id.clone()));
        payment_active.status = Set(map_payment_status(payment_status));
        payment_active.xendit_status = Set(Some(payload.status.clone()));
        payment_active.payment_method = Set(Some(payload.payment_method.clone()));
        payment_active.payment_channel = Set(Some(payload.payment_channel.clone()));
        payment_active.paid_amount = Set(Some(payload.paid_amount as f64));

        if matches!(payment_status, PaymentStatus::Paid | PaymentStatus::Settled) {
            payment_active.paid_at = Set(Some(chrono::Utc::now().naive_utc()));
        }

        payment_active
            .update(&txn)
            .await
            .map_err(|e| format!("Failed to update payment: {}", e))?
    } else {
        info!("Creating new payment record");

        let new_payment = payment::ActiveModel {
            order_id: Set(order_id),
            amount: Set(payload.amount as f64),
            method: Set(payload.payment_channel.clone()),
            status: Set(map_payment_status(payment_status)),
            paid_at: Set(if matches!(payment_status, PaymentStatus::Paid | PaymentStatus::Settled) {
                Some(chrono::Utc::now().naive_utc())
            } else {
                None
            }),
            external_id: Set(Some(payload.external_id.clone())),
            invoice_id: Set(Some(payload.id.clone())),
            invoice_url: Set(None),
            xendit_status: Set(Some(payload.status.clone())),
            payment_channel: Set(Some(payload.payment_channel.clone())),
            paid_amount: Set(Some(payload.paid_amount as f64)),
            payment_method: Set(Some(payload.payment_method.clone())),
            xendit_fees: Set(None),
            expiry_date: Set(None),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
            updated_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        new_payment
            .insert(&txn)
            .await
            .map_err(|e| format!("Failed to create payment: {}", e))?
    };

    let new_order_status = map_order_status(payment_status);
    info!("Updating order #{} from '{}' to '{}'", order.id, order.status, new_order_status);

    let mut order_active: order::ActiveModel = order.into();
    order_active.status = Set(new_order_status.to_string());

    let updated_order = order_active
        .update(&txn)
        .await
        .map_err(|e| format!("Failed to update order: {}", e))?;

    txn.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    info!("Transaction committed successfully");

    Ok((payment, updated_order))
}

fn map_payment_status(status: &PaymentStatus) -> String {
    match status {
        PaymentStatus::Paid => "paid".to_string(),
        PaymentStatus::Settled => "settled".to_string(),
        PaymentStatus::Expired => "expired".to_string(),
        PaymentStatus::Failed => "failed".to_string(),
        PaymentStatus::Pending => "pending".to_string(),
    }
}

fn map_order_status(status: &PaymentStatus) -> &'static str {
    match status {
        PaymentStatus::Paid | PaymentStatus::Settled => "paid",
        PaymentStatus::Expired => "expired",
        PaymentStatus::Failed => "failed",
        PaymentStatus::Pending => "pending",
    }
}

async fn handle_post_payment_actions(
    payment_status: &PaymentStatus,
    order_id: i64,
    payload: &XenditWebhookPayload,
) -> Result<(), String> {
    match payment_status {
        PaymentStatus::Paid | PaymentStatus::Settled => {
            info!("PAYMENT SUCCESS - Order #{}", order_id);
            info!("  Amount: Rp {}", payload.paid_amount);
            info!("  Channel: {}", payload.payment_channel);
            Ok(())
        }
        PaymentStatus::Expired => {
            warn!("PAYMENT EXPIRED - Order #{}", order_id);
            Ok(())
        }
        PaymentStatus::Failed => {
            error!("PAYMENT FAILED - Order #{}", order_id);
            Ok(())
        }
        PaymentStatus::Pending => {
            info!("PAYMENT PENDING - Order #{}", order_id);
            Ok(())
        }
    }
}

fn success_response(payment_id: i64, payment_status: &str, order_status: &str) -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Webhook processed successfully",
            "data": {
                "payment_id": payment_id,
                "payment_status": payment_status,
                "order_status": order_status
            }
        })),
    )
        .into_response()
}

fn error_response(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(json!({
            "success": false,
            "message": message
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_order_id() {
        assert_eq!(extract_order_id("ORDER-1").unwrap(), 1);
        assert_eq!(extract_order_id("ORDER-123").unwrap(), 123);
        assert!(extract_order_id("INVALID-123").is_err());
        assert!(extract_order_id("ORDER-abc").is_err());
    }

    #[test]
    fn test_map_payment_status() {
        assert_eq!(map_payment_status(&PaymentStatus::Paid), "paid");
        assert_eq!(map_payment_status(&PaymentStatus::Settled), "settled");
        assert_eq!(map_payment_status(&PaymentStatus::Expired), "expired");
    }

    #[test]
    fn test_map_order_status() {
        assert_eq!(map_order_status(&PaymentStatus::Paid), "paid");
        assert_eq!(map_order_status(&PaymentStatus::Settled), "paid");
    }
}