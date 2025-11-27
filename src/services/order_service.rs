use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::models::{cart, cart_item, order, order_item, product, prelude::*};

pub struct OrderService;

impl OrderService {
    pub async fn create_from_cart( 
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<order::Model, String> {
        // Dapatkan cart user
        let cart = Cart::find()
            .filter(cart::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or("Cart tidak ditemukan")?;

        // Dapatkan items di cart
        let cart_items = CartItem::find()
            .filter(cart_item::Column::CartId.eq(cart.id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if cart_items.is_empty() {
            return Err("Cart kosong".to_string());
        }

        // Hitung total price dan validasi stok
        let mut total_price = BigDecimal::from_i32(0).unwrap();

        for item in &cart_items {
            if let Some(product_id) = item.product_id {
                let product = Product::find_by_id(product_id)
                    .one(db)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?
                    .ok_or("Produk tidak ditemukan")?;

                if !product.is_active.unwrap_or(false) {
                    return Err("Produk tidak tersedia".to_string());
                }

                if product.stock < item.quantity {
                    return Err(format!("Stok {} tidak mencukupi", product.name));
                }

                // Perhitungan dengan BigDecimal
                let quantity_decimal = BigDecimal::from_i32(item.quantity).unwrap();
                let item_total = &product.price * quantity_decimal;
                total_price += item_total;
            }
        }

        // Buat order
        let new_order = order::ActiveModel {
            user_id: Set(user_id),
            total_price: Set(total_price),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let order = new_order
            .insert(db)
            .await
            .map_err(|e| format!("Gagal membuat order: {}", e))?;

        // Proses order items, update stok, dan kosongkan cart
        Self::process_order_items(db, &order, &cart_items).await?;

        Ok(order)
    }

    /// Helper untuk memproses order items
    async fn process_order_items(
        db: &DatabaseConnection,
        order: &order::Model,
        cart_items: &[cart_item::Model],
    ) -> Result<(), String> {
        for item in cart_items {
            if let Some(product_id) = item.product_id {
                let product = Product::find_by_id(product_id)
                    .one(db)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?
                    .ok_or("Produk tidak ditemukan")?;

                // Perhitungan subtotal dengan BigDecimal
                let quantity_decimal = BigDecimal::from_i32(item.quantity).unwrap();
                let subtotal = &product.price * quantity_decimal;

                // Buat order item
                let new_order_item = order_item::ActiveModel {
                    order_id: Set(order.id),
                    product_id: Set(Some(product_id)),
                    price: Set(product.price.clone()),
                    quantity: Set(item.quantity),
                    subtotal: Set(Some(subtotal)),
                    ..Default::default()
                };

                new_order_item
                    .insert(db)
                    .await
                    .map_err(|e| format!("Gagal menambah order item: {}", e))?;

                // Update stok produk
                let mut product_active: product::ActiveModel = product.into();
                product_active.stock = Set(product_active.stock.take().unwrap() - item.quantity);
                product_active
                    .update(db)
                    .await
                    .map_err(|e| format!("Gagal update stok produk: {}", e))?;
            }
        }

        // Kosongkan cart setelah order berhasil
        CartItem::delete_many()
            .filter(cart_item::Column::CartId.eq(cart_items.first().unwrap().cart_id))
            .exec(db)
            .await
            .map_err(|e| format!("Gagal mengosongkan cart: {}", e))?;

        Ok(())
    }

    /// Get order by ID (konsisten dengan service lain)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        order_id: i64,
    ) -> Result<Option<order::Model>, String> {
        Order::find_by_id(order_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all orders for a user
    pub async fn get_user_orders(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<order::Model>, String> {
        Order::find()
            .filter(order::Column::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all orders (admin only)
    pub async fn get_all_orders(
        db: &DatabaseConnection,
    ) -> Result<Vec<order::Model>, String> {
        Order::find()
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Update order status
    pub async fn update_status(
        db: &DatabaseConnection,
        order_id: i64,
        new_status: String,
    ) -> Result<Option<order::Model>, String> {
        let order = Order::find_by_id(order_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(order) = order {
            let mut order_active: order::ActiveModel = order.into();
            order_active.status = Set(new_status);

            let updated = order_active
                .update(db)
                .await
                .map_err(|e| format!("Failed to update order: {}", e))?;

            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(item) = item {
            let item_active: order_item::ActiveModel = item.into();
            item_active
                .delete(db)
                .await
                .map_err(|e| format!("Failed to delete order item: {}", e))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}