use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::models::{order, user::UserRole};
use crate::services::{CartService, OrderService};
use crate::utils::jwt;

#[derive(Default)]
pub struct OrderMutations;

#[Object]
impl OrderMutations {
    /// Create order from cart items
    async fn create_order(&self, ctx: &Context<'_>, token: String) -> Result<order::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let cart_items = CartService::get_cart_items(db, claims.user_id)
            .await
            .map_err(Error::new)?;

        if cart_items.is_empty() {
            return Err(Error::new("Cart is empty. Add items to cart before creating order."));
        }

        OrderService::create_from_cart(db, claims.user_id)
            .await
            .map_err(Error::new)
    }

    /// Update order status (Admin only or system callback)
    async fn update_order_status(
        &self,
        ctx: &Context<'_>,
        token: String,
        order_id: i64,
        status: String,
    ) -> Result<order::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin {
            return Err(Error::new("Unauthorized: Only Admin can update order status"));
        }

        let valid_statuses = vec!["pending", "processing", "shipped", "delivered", "cancelled"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(Error::new(format!(
                "Invalid status. Valid statuses: {}",
                valid_statuses.join(", ")
            )));
        }

        let updated = OrderService::update_status(db, order_id, status)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Order not found"))?;

        Ok(updated)
    }
}
