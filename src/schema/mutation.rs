use async_graphql::*;
use sea_orm::DatabaseConnection;

use crate::models::{cart_item, order, product, review, user};
use crate::models::user::UserRole;
use crate::services::{CartService, OrderService, ProductService, ReviewService, UserService};
use crate::utils::jwt;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ==================== AUTH MUTATIONS ====================
    
    /// Register user baru
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
        role: Option<UserRole>,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        let user_role = role.unwrap_or(UserRole::Pengguna);
        
        let user = UserService::create_user(db, username, email, password, user_role)
            .await
            .map_err(Error::new)?;
        
        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;
        
        Ok(AuthResponse { token, user })
    }
    
    /// Login user
    async fn login(
        &self,
        ctx: &Context<'_>,
        username_or_email: String,
        password: String,
    ) -> Result<AuthResponse> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        let user = UserService::verify_password(db, &username_or_email, &password)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Username/email atau password salah"))?;
        
        let token = jwt::create_jwt(user.id, &user.email, &user.username, user.role)
            .map_err(Error::new)?;
        
        Ok(AuthResponse { token, user })
    }

    // ==================== PRODUCT MUTATIONS ====================
    
    /// Tambah produk baru
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        name: String,
        slug: String,
        description: Option<String>,
        price: String,
        stock: i32,
        category_id: Option<i64>,
        seller_id: Option<i64>,
    ) -> Result<product::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        ProductService::create(db, name, slug, description, price, stock, category_id, seller_id)
            .await
            .map_err(Error::new)
    }
    
    /// Update produk
    async fn update_product(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        slug: Option<String>,
        description: Option<String>,
        price: Option<String>,
        stock: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Option<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        ProductService::update(db, id, name, slug, description, price, stock, is_active)
            .await
            .map_err(Error::new)
    }
    
    /// Soft delete produk (set is_active = false)
    async fn delete_product(&self, ctx: &Context<'_>, id: i64) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::soft_delete(db, id).await.map_err(Error::new)
    }
    
    /// Hard delete produk (hapus permanen dari database)
    async fn hard_delete_product(&self, ctx: &Context<'_>, id: i64) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::hard_delete(db, id).await.map_err(Error::new)
    }

    // ==================== CART MUTATIONS ====================
    
    /// Tambah item ke cart
    async fn add_to_cart(
        &self,
        ctx: &Context<'_>,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<cart_item::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        CartService::add_item(db, user_id, product_id, quantity)
            .await
            .map_err(Error::new)
    }
    
    /// Update quantity item di cart
    async fn update_cart_item_quantity(
        &self,
        ctx: &Context<'_>,
        item_id: i64,
        quantity: i32,
    ) -> Result<Option<cart_item::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        CartService::update_item_quantity(db, item_id, quantity)
            .await
            .map_err(Error::new)
    }
    
    /// Hapus item dari cart
    async fn remove_from_cart(&self, ctx: &Context<'_>, item_id: i64) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        CartService::remove_item(db, item_id).await.map_err(Error::new)
    }

    // ==================== ORDER MUTATIONS ====================
    
    /// Buat order dari cart (checkout)
    async fn create_order(&self, ctx: &Context<'_>, user_id: i64) -> Result<order::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        OrderService::create_from_cart(db, user_id)
            .await
            .map_err(Error::new)
    }

    // ==================== REVIEW MUTATIONS ====================
    
    /// Buat review untuk produk
    async fn create_review(
        &self,
        ctx: &Context<'_>,
        user_id: i64,
        product_id: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<review::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        ReviewService::create_review(db, user_id, product_id, rating, comment)
            .await
            .map_err(Error::new)
    }
    
    /// Update review
    async fn update_review(
        &self,
        ctx: &Context<'_>,
        review_id: i64,
        rating: Option<i32>,
        comment: Option<String>,
    ) -> Result<Option<review::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        ReviewService::update_review(db, review_id, rating, comment)
            .await
            .map_err(Error::new)
    }
    
    /// Hapus review
    async fn delete_review(&self, ctx: &Context<'_>, review_id: i64) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        ReviewService::delete_review(db, review_id).await.map_err(Error::new)
    }

    // ==================== USER MUTATIONS ====================
    
    /// Update user profile
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: i64,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<Option<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        UserService::update_user(db, user_id, username, email, password, None)
            .await
            .map_err(Error::new)
    }
    
    /// Delete user (admin only)
    async fn delete_user(&self, ctx: &Context<'_>, user_id: i64) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserService::delete_user(db, user_id).await.map_err(Error::new)
    }
}

// ==================== RESPONSE TYPES ====================

#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
    pub user: user::Model,
}