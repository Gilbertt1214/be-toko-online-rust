use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::models::{cart, cart_item, prelude::*};

pub struct CartService;

impl CartService {
    /// Get existing cart or create new one for user
    pub async fn get_or_create_cart(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<cart::Model, String> {
        // Check if cart exists
        if let Some(cart) = Cart::find()
            .filter(cart::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        {
            return Ok(cart);
        }

        // Validate user exists
        let user_exists = User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .is_some();

        if !user_exists {
            return Err("User not found".to_string());
        }

        // Create new cart
        let new_cart = cart::ActiveModel {
            user_id: Set(user_id),
            ..Default::default()
        };

        new_cart
            .insert(db)
            .await
            .map_err(|e| format!("Failed to create cart: {}", e))
    }

    /// Add item to cart or update quantity if exists
    pub async fn add_item(
        db: &DatabaseConnection,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<cart_item::Model, String> {
        // Validate product
        let product = Product::find_by_id(product_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or("Product not found")?;

        if !product.is_active.unwrap_or(false) {
            return Err("Product is not available".to_string());
        }

        if product.stock < quantity {
            return Err("Insufficient stock".to_string());
        }

        // Get or create cart
        let cart = Self::get_or_create_cart(db, user_id).await?;

        // Check if item already exists in cart
        if let Some(existing_item) = CartItem::find()
            .filter(cart_item::Column::CartId.eq(cart.id))
            .filter(cart_item::Column::ProductId.eq(product_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        {
            let current_qty = existing_item.quantity;
            let new_quantity = current_qty + quantity;

            if product.stock < new_quantity {
                return Err("Insufficient stock".to_string());
            }

            let mut active: cart_item::ActiveModel = existing_item.into();
            active.quantity = Set(new_quantity);

            return active
                .update(db)
                .await
                .map_err(|e| format!("Failed to update cart: {}", e));
        }

        // Create new item
        let new_item = cart_item::ActiveModel {
            cart_id: Set(cart.id),
            product_id: Set(Some(product_id)),
            quantity: Set(quantity),
            ..Default::default()
        };

        new_item
            .insert(db)
            .await
            .map_err(|e| format!("Failed to add item: {}", e))
    }

    /// Update cart item quantity
    pub async fn update_item_quantity(
        db: &DatabaseConnection,
        item_id: i64,
        quantity: i32,
    ) -> Result<Option<cart_item::Model>, String> {
        if quantity <= 0 {
            CartItem::delete_by_id(item_id)
                .exec(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
            return Ok(None);
        }

        let Some(item) = CartItem::find_by_id(item_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        // Validate stock
        if let Some(product_id) = item.product_id {
            let product = Product::find_by_id(product_id)
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or("Product not found")?;

            if product.stock < quantity {
                return Err("Insufficient stock".to_string());
            }
        }

        let mut active: cart_item::ActiveModel = item.into();
        active.quantity = Set(quantity);

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Failed to update quantity: {}", e))
    }

    /// Remove item from cart
    pub async fn remove_item(db: &DatabaseConnection, item_id: i64) -> Result<bool, String> {
        let res = CartItem::delete_by_id(item_id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Get all cart items for a user
    pub async fn get_cart_items(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<cart_item::Model>, String> {
        let cart = Self::get_or_create_cart(db, user_id).await?;

        CartItem::find()
            .filter(cart_item::Column::CartId.eq(cart.id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Clear all items from user's cart
    pub async fn clear_cart(db: &DatabaseConnection, user_id: i64) -> Result<bool, String> {
        let cart = Self::get_or_create_cart(db, user_id).await?;

        let res = CartItem::delete_many()
            .filter(cart_item::Column::CartId.eq(cart.id))
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Get cart item count for user
    pub async fn get_cart_count(db: &DatabaseConnection, user_id: i64) -> Result<usize, String> {
        let items = Self::get_cart_items(db, user_id).await?;
        Ok(items.len())
    }
}