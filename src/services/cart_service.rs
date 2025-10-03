use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::models::{cart, cart_item, prelude::*};

pub struct CartService;

impl CartService {
    pub async fn get_or_create_cart(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<cart::Model, String> {
        // Cek cart yang sudah ada
        if let Some(cart) = Cart::find()
            .filter(cart::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        {
            return Ok(cart);
        }

        // Validasi user exists
        let user_exists = User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .is_some();

        if !user_exists {
            return Err("User tidak ditemukan".to_string());
        }

        // Buat cart baru
        let new_cart = cart::ActiveModel {
            user_id: Set(user_id),
            ..Default::default()
        };

        new_cart
            .insert(db)
            .await
            .map_err(|e| format!("Gagal membuat cart: {}", e))
    }

    pub async fn add_item(
        db: &DatabaseConnection,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<cart_item::Model, String> {
        // Validasi produk
        let product = Product::find_by_id(product_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or("Produk tidak ditemukan")?;

        if !product.is_active.unwrap_or(false) {
            return Err("Produk tidak tersedia".to_string());
        }

        if product.stock < quantity {
            return Err("Stok tidak mencukupi".to_string());
        }

        // Get or create cart
        let cart = Self::get_or_create_cart(db, user_id).await?;

        // Cek item sudah ada?
        if let Some(existing_item) = CartItem::find()
            .filter(cart_item::Column::CartId.eq(cart.id))
            .filter(cart_item::Column::ProductId.eq(product_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        {
            // PERBAIKAN: Cara yang benar untuk mengakses nilai dari ActiveModel
            let current_qty = existing_item.quantity; // Ambil dari Model, bukan ActiveModel
            let new_quantity = current_qty + quantity;
            
            let mut active: cart_item::ActiveModel = existing_item.into();
            active.quantity = Set(new_quantity);

            return active
                .update(db)
                .await
                .map_err(|e| format!("Gagal update cart: {}", e));
        }

        // Buat item baru
        let new_item = cart_item::ActiveModel {
            cart_id: Set(cart.id),
            product_id: Set(Some(product_id)),
            quantity: Set(quantity),
            ..Default::default()
        };

        new_item
            .insert(db)
            .await
            .map_err(|e| format!("Gagal menambah item: {}", e))
    }

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

        // Validasi stok
        if let Some(product_id) = item.product_id {
            let product = Product::find_by_id(product_id)
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or("Produk tidak ditemukan")?;

            if product.stock < quantity {
                return Err("Stok tidak mencukupi".to_string());
            }
        }

        let mut active: cart_item::ActiveModel = item.into();
        active.quantity = Set(quantity);

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Gagal update quantity: {}", e))
    }

    pub async fn remove_item(db: &DatabaseConnection, item_id: i64) -> Result<bool, String> {
        let res = CartItem::delete_by_id(item_id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }
    // Tambahkan method ini di impl CartService

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
}