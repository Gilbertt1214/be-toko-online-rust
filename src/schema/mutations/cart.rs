use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::models::{cart_item, prelude::*};
use crate::services::{CartService, ProductService};
use crate::utils::jwt;

#[derive(Default)]
pub struct CartMutations;

#[Object]
impl CartMutations {
    /// Add item to cart
    async fn add_to_cart(
        &self,
        ctx: &Context<'_>,
        token: String,
        product_id: i64,
        quantity: i32,
    ) -> Result<cart_item::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if quantity <= 0 {
            return Err(Error::new("Quantity harus lebih dari 0"));
        }

        let product = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        if !product.is_active.unwrap_or(false) {
            return Err(Error::new("Product is not available"));
        }
        if product.stock < quantity {
            return Err(Error::new(format!(
                "Insufficient stock. Available: {}, Requested: {}",
                product.stock, quantity
            )));
        }

        CartService::add_item(db, claims.user_id, product_id, quantity)
            .await
            .map_err(Error::new)
    }

    /// Update cart item quantity
    async fn update_cart_item(
        &self,
        ctx: &Context<'_>,
        token: String,
        cart_item_id: i64,
        quantity: i32,
    ) -> Result<cart_item::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if quantity <= 0 {
            return Err(Error::new("Quantity harus lebih dari 0"));
        }

        let cart_item = CartItem::find_by_id(cart_item_id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("Cart item not found"))?;

        let cart = Cart::find_by_id(cart_item.cart_id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("Cart not found"))?;

        if cart.user_id != claims.user_id {
            return Err(Error::new("Unauthorized"));
        }

        if let Some(product_id) = cart_item.product_id {
            let product = Product::find_by_id(product_id)
                .one(db)
                .await
                .map_err(|e| Error::new(format!("Database error: {}", e)))?
                .ok_or_else(|| Error::new("Product not found"))?;

            if product.stock < quantity {
                return Err(Error::new(format!(
                    "Insufficient stock. Available: {}, Requested: {}",
                    product.stock, quantity
                )));
            }
        }

        let mut cart_item_active: cart_item::ActiveModel = cart_item.into();
        cart_item_active.quantity = Set(quantity);

        cart_item_active
            .update(db)
            .await
            .map_err(|e| Error::new(format!("Failed to update cart item: {}", e)))
    }

    /// Remove item from cart
    async fn remove_from_cart(
        &self,
        ctx: &Context<'_>,
        token: String,
        cart_item_id: i64,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let _claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let success = CartService::remove_item(db, cart_item_id)
            .await
            .map_err(Error::new)?;

        if success {
            Ok("Item removed from cart".to_string())
        } else {
            Err(Error::new("Failed to remove item or item not found"))
        }
    }

    /// Clear all items from cart
    async fn clear_cart(&self, ctx: &Context<'_>, token: String) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        CartService::clear_cart(db, claims.user_id)
            .await
            .map_err(Error::new)?;

        Ok("Cart cleared successfully".to_string())
    }
}
