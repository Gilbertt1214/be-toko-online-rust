use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use crate::services::{UserService, OrderService, PaymentService, ProductService, CartService, ReviewService};
use crate::graphql::graphql_types::{UserGraphQL, OrderGraphQL, PaymentGraphQL, ProductGraphQL, CartItemGraphQL, ReviewGraphQL};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ==================== USER QUERIES ====================
    
    /// Get all users
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let users = UserService::get_all(db).await?;
        Ok(users.into_iter().map(UserGraphQL::from).collect())
    }

    /// Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: i64) -> Result<Option<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_id(db, id).await?;
        Ok(user.map(UserGraphQL::from))
    }

    // ==================== PRODUCT QUERIES ====================

    /// Get all products
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_all(db).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get product by ID
    async fn product(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let product = ProductService::get_by_id(db, id).await?;
        Ok(product.map(ProductGraphQL::from))
    }

    /// Get product by slug
    async fn product_by_slug(&self, ctx: &Context<'_>, slug: String) -> Result<Option<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let product = ProductService::get_by_slug(db, &slug).await?;
        Ok(product.map(ProductGraphQL::from))
    }

    /// Search products by keyword
    async fn search_products(&self, ctx: &Context<'_>, keyword: String) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::search(db, &keyword).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products by category
    async fn products_by_category(&self, ctx: &Context<'_>, category_id: i64) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_by_category(db, category_id).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products by seller
    async fn products_by_seller(&self, ctx: &Context<'_>, seller_id: i64) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_by_seller(db, seller_id).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get active products only
    async fn active_products(&self, ctx: &Context<'_>) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_active(db).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products with pagination
    async fn products_paginated(
        &self, 
        ctx: &Context<'_>, 
        page: i64, 
        page_size: i64,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_paginated(db, page as u64, page_size as u64, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get total product count
    async fn product_count(
        &self, 
        ctx: &Context<'_>,
        #[graphql(default = false)] active_only: bool
    ) -> Result<i64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = ProductService::count_all(db, active_only).await?;
        Ok(count as i64)
    }

    /// Get products by price range
    async fn products_by_price_range(
        &self,
        ctx: &Context<'_>,
        min_price: String,
        max_price: String,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_by_price_range(db, &min_price, &max_price, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products with low stock
    async fn low_stock_products(
        &self,
        ctx: &Context<'_>,
        threshold: i32
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_low_stock(db, threshold).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get out of stock products
    async fn out_of_stock_products(&self, ctx: &Context<'_>) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_out_of_stock(db).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get featured products (active products sorted by creation date)
    async fn featured_products(
        &self,
        ctx: &Context<'_>,
        limit: i64
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_featured(db, limit as u64).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get recent products
    async fn recent_products(
        &self,
        ctx: &Context<'_>,
        limit: i64,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_recent(db, limit as u64, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products sorted by price (ascending)
    async fn products_by_price_asc(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_sorted_by_price_asc(db, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products sorted by price (descending)
    async fn products_by_price_desc(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_sorted_by_price_desc(db, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products sorted by name
    async fn products_by_name(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_sorted_by_name(db, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products sorted by stock (ascending)
    async fn products_by_stock_asc(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_sorted_by_stock_asc(db, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    /// Get products sorted by stock (descending)
    async fn products_by_stock_desc(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_sorted_by_stock_desc(db, active_only).await?;
        Ok(products.into_iter().map(ProductGraphQL::from).collect())
    }

    // ==================== CART QUERIES ====================

    /// Get cart items for user
    async fn cart_items(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<CartItemGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cart_items = CartService::get_cart_items(db, user_id).await?;
        Ok(cart_items.into_iter().map(CartItemGraphQL::from).collect())
    }

    /// Get cart item count for user
    async fn cart_count(&self, ctx: &Context<'_>, user_id: i64) -> Result<usize> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = CartService::get_cart_count(db, user_id).await?;
        Ok(count)
    }

    // ==================== REVIEW QUERIES ====================

    /// Get review by ID
    async fn review(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let review = ReviewService::get_by_id(db, id).await?;
        Ok(review.map(ReviewGraphQL::from))
    }

    /// Get all reviews for a product
    async fn product_reviews(&self, ctx: &Context<'_>, product_id: i64) -> Result<Vec<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let reviews = ReviewService::get_by_product(db, product_id).await?;
        Ok(reviews.into_iter().map(ReviewGraphQL::from).collect())
    }

    /// Get all reviews by a user
    async fn user_reviews(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let reviews = ReviewService::get_by_user(db, user_id).await?;
        Ok(reviews.into_iter().map(ReviewGraphQL::from).collect())
    }

    /// Get user's review for a specific product
    async fn user_product_review(
        &self, 
        ctx: &Context<'_>, 
        user_id: i64, 
        product_id: i64
    ) -> Result<Option<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let review = ReviewService::get_user_review(db, user_id, product_id).await?;
        Ok(review.map(ReviewGraphQL::from))
    }

    /// Get average rating for a product
    async fn product_average_rating(&self, ctx: &Context<'_>, product_id: i64) -> Result<f64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let average_rating = ReviewService::get_average_rating(db, product_id).await?;
        Ok(average_rating)
    }

    /// Get review count for a product
    async fn product_review_count(&self, ctx: &Context<'_>, product_id: i64) -> Result<usize> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = ReviewService::get_review_count(db, product_id).await?;
        Ok(count)
    }

    // ==================== ORDER QUERIES ====================

    /// Get all orders
    async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let orders = OrderService::get_all_orders(db).await?;
        Ok(orders.into_iter().map(OrderGraphQL::from).collect())
    }

    /// Get order by ID
    async fn order(&self, ctx: &Context<'_>, id: i64) -> Result<Option<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let order = OrderService::get_by_id(db, id).await?;
        Ok(order.map(OrderGraphQL::from))
    }

    /// Get orders by user ID
    async fn user_orders(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let orders = OrderService::get_user_orders(db, user_id).await?;
        Ok(orders.into_iter().map(OrderGraphQL::from).collect())
    }

    // ==================== PAYMENT QUERIES ====================

    /// Get payment invoice by Xendit invoice ID
    async fn payment_invoice(&self, ctx: &Context<'_>, invoice_id: String) -> Result<Option<PaymentGraphQL>> {
        let payment_service = ctx.data::<PaymentService>()?;
        
        match payment_service.get_invoice(&invoice_id).await {
            Ok(invoice) => Ok(Some(PaymentGraphQL::from(invoice))),
            Err(_) => Ok(None),
        }
    }

    /// Get payment invoice by external ID (order ID)
    async fn payment_invoice_by_order(&self, ctx: &Context<'_>, order_id: String) -> Result<Option<PaymentGraphQL>> {
        let payment_service = ctx.data::<PaymentService>()?;
        
        match payment_service.get_invoice_by_external_id(&order_id).await {
            Ok(invoice) => Ok(Some(PaymentGraphQL::from(invoice))),
            Err(_) => Ok(None),
        }
    }

    /// Get payment status by order ID
    async fn payment_status(&self, ctx: &Context<'_>, order_id: String) -> Result<String> {
        let payment_service = ctx.data::<PaymentService>()?;
        
        match payment_service.get_invoice_by_external_id(&order_id).await {
            Ok(invoice) => Ok(invoice.status),
            Err(e) => Err(async_graphql::Error::new(format!("Failed to get payment status: {}", e))),
        }
    }

    /// Get all payment invoices for a user by email
    async fn user_payments(&self, ctx: &Context<'_>, user_email: String) -> Result<Vec<PaymentGraphQL>> {
        let payment_service = ctx.data::<PaymentService>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        
        let user = UserService::get_by_email(db, &user_email).await?;
        if let Some(user) = user {
            let orders = OrderService::get_user_orders(db, user.id).await?;
            
            let mut payments = Vec::new();
            for order in orders {
                if let Ok(invoice) = payment_service.get_invoice_by_external_id(&order.id.to_string()).await {
                    payments.push(PaymentGraphQL::from(invoice));
                }
            }
            
            Ok(payments)
        } else {
            Ok(vec![])
        }
    }

    /// Get payment invoices by status
    async fn payments_by_status(&self, ctx: &Context<'_>, status: String) -> Result<Vec<PaymentGraphQL>> {
        let payment_service = ctx.data::<PaymentService>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        
        let orders = OrderService::get_all_orders(db).await?;
        
        let mut payments = Vec::new();
        for order in orders {
            if let Ok(invoice) = payment_service.get_invoice_by_external_id(&order.id.to_string()).await {
                if invoice.status.to_uppercase() == status.to_uppercase() {
                    payments.push(PaymentGraphQL::from(invoice));
                }
            }
        }
        
        Ok(payments)
    }

    /// Verify if payment service is available
    async fn payment_service_status(&self, ctx: &Context<'_>) -> Result<bool> {
        let _payment_service = ctx.data::<PaymentService>()?;
        Ok(true)
    }

    /// Get payment service configuration info
    async fn payment_service_info(&self, ctx: &Context<'_>) -> Result<String> {
        let payment_service = ctx.data::<PaymentService>()?;
        
        let mode = if payment_service.is_production() {
            "PRODUCTION"
        } else {
            "SANDBOX"
        };
        
        Ok(format!("Xendit Payment Service - Mode: {}", mode))
    }
}