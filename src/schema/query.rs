use async_graphql::*;
use sea_orm::DatabaseConnection;

use crate::models::{cart, cart_item, order, product, review, user};
use crate::services::{CartService, OrderService, ProductService, ReviewService, UserService};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ==================== PRODUCT QUERIES ====================
    
    /// Get all products
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::get_all_products(db).await.map_err(Error::new)
    }
    
    /// Get product by ID
    async fn product(&self, ctx: &Context<'_>, id: i64) -> Result<Option<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::get_product_by_id(db, id).await.map_err(Error::new)
    }
    
    /// Get product by slug
    async fn product_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> Result<Option<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::get_product_by_slug(db, &slug).await.map_err(Error::new)
    }
    
    /// Search products by name
    async fn search_products(
        &self,
        ctx: &Context<'_>,
        keyword: String,
    ) -> Result<Vec<product::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ProductService::search_products(db, &keyword).await.map_err(Error::new)
    }

    // ==================== USER QUERIES ====================
    
    /// Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: i64) -> Result<Option<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserService::get_user_by_id(db, id).await.map_err(Error::new)
    }
    
    /// Get user by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> Result<Option<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserService::get_user_by_username(db, &username).await.map_err(Error::new)
    }
    
    /// Get all users (admin only - add auth check in production)
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserService::get_all_users(db).await.map_err(Error::new)
    }

    // ==================== CART QUERIES ====================
    
    /// Get or create user's cart
    async fn my_cart(&self, ctx: &Context<'_>, user_id: i64) -> Result<cart::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        CartService::get_or_create_cart(db, user_id).await.map_err(Error::new)
    }
    
    /// Get cart items for a user
    async fn cart_items(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<cart_item::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        CartService::get_cart_items(db, user_id).await.map_err(Error::new)
    }

    // ==================== ORDER QUERIES ====================
    
    /// Get user's orders
    async fn my_orders(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<order::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        OrderService::get_user_orders(db, user_id).await.map_err(Error::new)
    }
    
    /// Get order by ID
    async fn order(&self, ctx: &Context<'_>, order_id: i64) -> Result<Option<order::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        OrderService::get_order_by_id(db, order_id).await.map_err(Error::new)
    }
    
    /// Get all orders (admin only - add auth check in production)
    async fn all_orders(&self, ctx: &Context<'_>) -> Result<Vec<order::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        OrderService::get_all_orders(db).await.map_err(Error::new)
    }

    // ==================== REVIEW QUERIES ====================
    
    /// Get reviews for a product
    async fn product_reviews(
        &self,
        ctx: &Context<'_>,
        product_id: i64,
    ) -> Result<Vec<review::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ReviewService::get_reviews_by_product(db, product_id).await.map_err(Error::new)
    }
    
    /// Get reviews by user
    async fn user_reviews(
        &self,
        ctx: &Context<'_>,
        user_id: i64,
    ) -> Result<Vec<review::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ReviewService::get_reviews_by_user(db, user_id).await.map_err(Error::new)
    }
    
    /// Get single review by ID
    async fn review(&self, ctx: &Context<'_>, review_id: i64) -> Result<Option<review::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        ReviewService::get_review_by_id(db, review_id).await.map_err(Error::new)
    }
    
    /// Get average rating for product
    async fn product_rating(&self, ctx: &Context<'_>, product_id: i64) -> Result<f64> {
        let db = ctx.data::<DatabaseConnection>()?;
        ReviewService::get_average_rating(db, product_id).await.map_err(Error::new)
    }
}