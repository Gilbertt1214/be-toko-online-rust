use async_graphql::*;
use bigdecimal::ToPrimitive;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use std::sync::Arc;

use crate::models::user::UserRole;
use crate::models::{cart_item, order, product, review, user, order_item, prelude::*};
use crate::services::payment_service::PaymentService;
use crate::services::{CartService, OrderService, ProductService, ReviewService, UserService};
use crate::utils::jwt;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ==================== AUTH MUTATIONS ====================
    
    /// Register new user
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
        role: Option<UserRole>,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        if username.trim().is_empty() {
            return Err(Error::new("Username tidak boleh kosong"));
        }
        if email.trim().is_empty() || !email.contains('@') {
            return Err(Error::new("Email tidak valid"));
        }
        if password.len() < 6 {
            return Err(Error::new("Password minimal 6 karakter"));
        }

        let user_role = role.unwrap_or(UserRole::Pengguna);
        if user_role != UserRole::Pengguna {
            return Err(Error::new("Cannot register as Admin or Pengusaha through this endpoint"));
        }

        let user = UserService::create_user(db, username, email, password, user_role)
            .await
            .map_err(Error::new)?;

        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;

        Ok(AuthResponse { token, user })
    }

    /// Login with username/email and password
    async fn login(
        &self,
        ctx: &Context<'_>,
        username_or_email: String,
        password: String,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;

        if username_or_email.trim().is_empty() {
            return Err(Error::new("Username/email tidak boleh kosong"));
        }
        if password.is_empty() {
            return Err(Error::new("Password tidak boleh kosong"));
        }

        let user = UserService::verify_password(db, &username_or_email, &password)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Username/email atau password salah"))?;

        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;

        Ok(AuthResponse { token, user })
    }

    // ==================== PRODUCT MUTATIONS ====================
    
    /// Create new product (Admin/Pengusaha only)
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        token: String,
        name: String,
        slug: String,
        short_description: Option<String>,
        description: Option<String>,
        price: String,
        stock: i32,
        category_id: Option<i64>,
    ) -> Result<product::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin && claims.role != UserRole::Pengusaha {
            return Err(Error::new("Unauthorized: Only Admin and Pengusaha can create products"));
        }

        // Validations
        if name.trim().is_empty() {
            return Err(Error::new("Product name cannot be empty"));
        }
        if slug.trim().is_empty() {
            return Err(Error::new("Slug cannot be empty"));
        }
        if stock < 0 {
            return Err(Error::new("Stock tidak boleh negatif"));
        }

        let price_value: f64 = price.parse().map_err(|_| Error::new("Invalid price format"))?;
        if price_value < 0.0 {
            return Err(Error::new("Price tidak boleh negatif"));
        }

        // Validate short_description length (recommended max 200 chars)
        if let Some(ref short_desc) = short_description {
            if short_desc.len() > 500 {
                return Err(Error::new("Short description too long (max 500 characters)"));
            }
        }

        let seller_id = if claims.role == UserRole::Pengusaha {
            Some(claims.user_id)
        } else {
            None
        };

        ProductService::create(
            db,
            name,
            slug,
            short_description,
            description,
            price,
            stock,
            category_id,
            seller_id,
        )
        .await
        .map_err(Error::new)
    }

    /// Update existing product (Admin/Pengusaha only)
    async fn update_product(
        &self,
        ctx: &Context<'_>,
        token: String,
        product_id: i64,
        name: Option<String>,
        slug: Option<String>,
        short_description: Option<String>,
        description: Option<String>,
        price: Option<String>,
        stock: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Option<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin && claims.role != UserRole::Pengusaha {
            return Err(Error::new("Unauthorized: Only Admin and Pengusaha can update products"));
        }

        // Get existing product to verify ownership
        let existing = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        // Pengusaha can only update their own products
        if claims.role == UserRole::Pengusaha {
            if existing.seller_id != Some(claims.user_id) {
                return Err(Error::new("Unauthorized: You can only update your own products"));
            }
        }

        // Validations
        if let Some(ref n) = name {
            if n.trim().is_empty() {
                return Err(Error::new("Product name cannot be empty"));
            }
        }
        if let Some(ref s) = slug {
            if s.trim().is_empty() {
                return Err(Error::new("Slug cannot be empty"));
            }
        }
        if let Some(s) = stock {
            if s < 0 {
                return Err(Error::new("Stock tidak boleh negatif"));
            }
        }
        if let Some(ref p) = price {
            let price_value: f64 = p.parse().map_err(|_| Error::new("Invalid price format"))?;
            if price_value < 0.0 {
                return Err(Error::new("Price tidak boleh negatif"));
            }
        }
        if let Some(ref short_desc) = short_description {
            if short_desc.len() > 500 {
                return Err(Error::new("Short description too long (max 500 characters)"));
            }
        }

        ProductService::update(
            db,
            product_id,
            name,
            slug,
            short_description.map(Some),
            description.map(Some),
            price,
            stock,
            is_active,
        )
        .await
        .map_err(Error::new)
    }

    /// Delete product (Admin/Pengusaha only)
    async fn delete_product(
        &self,
        ctx: &Context<'_>,
        token: String,
        product_id: i64,
        hard_delete: Option<bool>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin && claims.role != UserRole::Pengusaha {
            return Err(Error::new("Unauthorized"));
        }

        // Get existing product to verify ownership
        let existing = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        // Pengusaha can only delete their own products
        if claims.role == UserRole::Pengusaha {
            if existing.seller_id != Some(claims.user_id) {
                return Err(Error::new("Unauthorized: You can only delete your own products"));
            }
        }

        let success = if hard_delete.unwrap_or(false) {
            // Only admin can hard delete
            if claims.role != UserRole::Admin {
                return Err(Error::new("Unauthorized: Only Admin can permanently delete products"));
            }
            ProductService::hard_delete(db, product_id)
                .await
                .map_err(Error::new)?
        } else {
            // Soft delete (set is_active = false)
            ProductService::soft_delete(db, product_id)
                .await
                .map_err(Error::new)?
        };

        if success {
            Ok(format!("Product {} deleted successfully", product_id))
        } else {
            Err(Error::new("Failed to delete product"))
        }
    }

    // ==================== CART MUTATIONS ====================
    
    /// Add product to cart
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

    /// Remove item from cart
    async fn remove_from_cart(
        &self,
        ctx: &Context<'_>,
        token: String,
        cart_item_id: i64,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let success = CartService::remove_item(db,  cart_item_id)
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

    // ==================== ORDER MUTATIONS ====================
    
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

    // ==================== PAYMENT MUTATIONS ====================
    
    /// Create payment invoice for order
    async fn create_payment_invoice(
        &self,
        ctx: &Context<'_>,
        token: String,
        order_id: i64,
        customer_phone: Option<String>,
    ) -> Result<PaymentInvoiceResponse> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        // Verify token
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        // Get order
        let order = OrderService::get_by_id(db, order_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Order not found"))?;

        // Verify ownership
        if order.user_id != claims.user_id {
            return Err(Error::new("Unauthorized: This is not your order"));
        }

        // Check order status
        if order.status.to_lowercase() != "pending" {
            return Err(Error::new(format!(
                "Order is not in pending status (current: {})",
                order.status
            )));
        }

        // Get user info
        let user = UserService::get_by_id(db, claims.user_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("User not found"))?;

        // Get order items with products
        let items_with_product = OrderItem::find()
            .filter(order_item::Column::OrderId.eq(order.id))
            .find_also_related(Product)
            .all(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;

        // Validate order has items
        if items_with_product.is_empty() {
            return Err(Error::new("Order has no items"));
        }

        // Build invoice items
        let items: Vec<_> = items_with_product
            .into_iter()
            .map(|(item, product)| {
                // Convert BigDecimal to i64 (Rupiah, bukan cents)
                let price_f64: f64 = item.price.to_f64().unwrap_or(0.0);
                let price_idr = price_f64.round() as i64;
                
                crate::services::payment_service::InvoiceItem {
                    name: product.map(|p| p.name).unwrap_or_else(|| "Product".to_string()),
                    quantity: item.quantity,
                    price: price_idr,
                    category: Some("Product".to_string()),
                }
            })
            .collect();

        // Calculate total amount in Rupiah
        let total_amount_f64 = order
            .total_price
            .to_f64()
            .ok_or_else(|| Error::new("Invalid total amount"))?;
        
        let amount = total_amount_f64.round() as i64;

        // Validate amount
        if amount <= 0 {
            return Err(Error::new("Invalid order amount"));
        }

        // Generate external ID
        let external_id = format!("ORDER-{}", order_id);

        // Build request
        let request = crate::services::payment_service::CreateInvoiceRequest {
            external_id: external_id.clone(),
            amount,
            payer_email: user.email.clone(),
            description: format!("Payment for Order #{} - {} items", order_id, items.len()),
            customer: crate::services::payment_service::CustomerInfo {
                given_names: user.username.clone(),
                email: user.email.clone(),
                mobile_number: customer_phone,
                address: None,
            },
            items,
            invoice_duration: Some(86400), // 24 jam
        };

        // Get payment service
        let payment_service = ctx.data::<Arc<PaymentService>>()?;

        // Create invoice via Xendit
        let invoice = payment_service
            .create_invoice(request)
            .await
            .map_err(|e| Error::new(format!("Failed to create invoice: {}", e)))?;

        Ok(PaymentInvoiceResponse {
            invoice_id: invoice.id,
            external_id: invoice.external_id,
            invoice_url: invoice.invoice_url,
            amount: total_amount_f64,
            status: invoice.status,
            expiry_date: invoice.expiry_date,
        })
    }

    /// Check payment status
    async fn check_payment_status(
        &self,
        ctx: &Context<'_>,
        token: String,
        invoice_id: String,
    ) -> Result<PaymentStatusResponse> {
        let _claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let payment_service = ctx.data::<Arc<PaymentService>>()?;

        let invoice = payment_service
            .get_invoice(&invoice_id)
            .await
            .map_err(|e| Error::new(format!("Failed to get invoice: {}", e)))?;

        Ok(PaymentStatusResponse {
            invoice_id: invoice.id,
            external_id: invoice.external_id,
            status: invoice.status,
            amount: invoice.amount as f64,
            paid_amount: invoice.paid_amount as f64,
        })
    }

    /// Expire payment invoice manually
    async fn expire_payment_invoice(
        &self,
        ctx: &Context<'_>,
        token: String,
        invoice_id: String,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let payment_service = ctx.data::<Arc<PaymentService>>()?;

        // Get invoice to verify ownership
        let invoice = payment_service
            .get_invoice(&invoice_id)
            .await
            .map_err(|e| Error::new(format!("Failed to get invoice: {}", e)))?;

        // Extract order_id from external_id (format: ORDER-123)
        let order_id: i64 = invoice
            .external_id
            .strip_prefix("ORDER-")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| Error::new("Invalid external_id format"))?;

        // Get order and verify ownership
        let order = OrderService::get_by_id(db, order_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Order not found"))?;

        if order.user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(Error::new("Unauthorized"));
        }

        // Expire invoice
        payment_service
            .expire_invoice(&invoice_id)
            .await
            .map_err(|e| Error::new(format!("Failed to expire invoice: {}", e)))?;

        Ok(format!("Invoice {} has been expired", invoice_id))
    }

    // ==================== REVIEW MUTATIONS ====================
    
    /// Add review for a product
    async fn add_review(
        &self,
        ctx: &Context<'_>,
        token: String,
        product_id: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<review::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if rating < 1 || rating > 5 {
            return Err(Error::new("Rating must be between 1 and 5"));
        }

        let product = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        if !product.is_active.unwrap_or(false) {
            return Err(Error::new("Cannot review an inactive product"));
        }

        ReviewService::create_review(db, claims.user_id, product_id, rating, comment)
            .await
            .map_err(Error::new)
    }

    /// Update existing review
    async fn update_review(
        &self,
        ctx: &Context<'_>,
        token: String,
        review_id: i64,
        rating: Option<i32>,
        comment: Option<String>,
    ) -> Result<Option<review::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if let Some(r) = rating {
            if r < 1 || r > 5 {
                return Err(Error::new("Rating must be between 1 and 5"));
            }
        }

        // Get existing review to verify ownership
        let existing = ReviewService::get_by_id(db, review_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Review not found"))?;

        if existing.user_id != claims.user_id {
            return Err(Error::new("Unauthorized: You can only update your own reviews"));
        }

        ReviewService::update_review(db, review_id, rating, comment)
            .await
            .map_err(Error::new)
    }

    /// Delete review
    async fn delete_review(
        &self,
        ctx: &Context<'_>,
        token: String,
        review_id: i64,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        // Get existing review to verify ownership
        let existing = ReviewService::get_by_id(db, review_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Review not found"))?;

        if existing.user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(Error::new("Unauthorized: You can only delete your own reviews"));
        }

        let success = ReviewService::delete_review(db, review_id)
            .await
            .map_err(Error::new)?;

        if success {
            Ok("Review deleted successfully".to_string())
        } else {
            Err(Error::new("Failed to delete review"))
        }
    }
}

// ==================== RESPONSE TYPES ====================

#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
    pub user: user::Model,
}

#[derive(SimpleObject)]
pub struct PaymentInvoiceResponse {
    pub invoice_id: String,
    pub external_id: String,
    pub invoice_url: String,
    pub amount: f64,
    pub status: String,
    pub expiry_date: String,
}

#[derive(SimpleObject)]
pub struct PaymentStatusResponse {
    pub invoice_id: String,
    pub external_id: String,
    pub status: String,
    pub amount: f64,
    pub paid_amount: f64,
}