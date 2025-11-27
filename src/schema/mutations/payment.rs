use async_graphql::*;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use bigdecimal::ToPrimitive;
use crate::graphql::graphql_types::PaymentInvoiceResponse;
use crate::models::order;
use crate::services::{OrderService, payment_service::*};
use crate::utils::jwt;

#[derive(Default)]
pub struct PaymentMutations;

#[Object]
impl PaymentMutations {
    /// Create payment invoice for an order
    async fn create_payment(
        &self,
        ctx: &Context<'_>,
        token: String,
        order_id: i64,
    ) -> Result<PaymentInvoiceResponse> {
        let db = ctx.data::<DatabaseConnection>()?;
        let payment_service = ctx.data::<Arc<PaymentService>>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let order_data: order::Model = OrderService::get_by_id(db, order_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Order not found"))?;

        if order_data.user_id != claims.user_id {
            return Err(Error::new("Unauthorized: This is not your order"));
        }

        if order_data.status != "pending" {
            return Err(Error::new("Payment already created or order is not pending"));
        }

        let order_items = OrderService::get_order_items(db, order_id)
            .await
            .map_err(Error::new)?;

        if order_items.is_empty() {
            return Err(Error::new("Order has no items"));
        }

        let external_id = format!("ORDER-{}", order_id);
        let amount = order_data.total_price
            .to_f64()
            .ok_or_else(|| Error::new("Invalid price conversion"))?;
        let amount_idr = (amount as i64).max(10000);

        let items: Vec<InvoiceItem> = order_items
            .iter()
            .map(|item| InvoiceItem {
                name: format!("Product ID {}", item.product_id.unwrap_or(0)),
                quantity: item.quantity,
                price: item.price.to_f64().unwrap_or(0.0) as i64,
                category: Some("Product".to_string()),
            })
            .collect();

        let invoice_request = CreateInvoiceRequest {
            external_id: external_id.clone(),
            amount: amount_idr,
            payer_email: claims.email.clone(),
            description: format!("Payment for Order #{}", order_id),
            customer: CustomerInfo {
                given_names: claims.username.clone(),
                email: claims.email.clone(),
                mobile_number: None,
                address: None,
            },
            items,
            invoice_duration: Some(86400),
        };

        let invoice = payment_service
            .create_invoice(invoice_request)
            .await
            .map_err(|e| Error::new(format!("Failed to create invoice: {}", e)))?;

        Ok(PaymentInvoiceResponse {
            invoice_id: invoice.id,
            external_id: external_id,
            invoice_url: invoice.invoice_url,
            amount: amount,
            status: invoice.status,
            expiry_date: invoice.expiry_date,
        })
    }
}
