use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use bigdecimal::ToPrimitive;
use crate::services::{UserService, OrderService, PaymentService, ProductService, CartService, ReviewService};
use crate::graphql::graphql_types::{
    UserGraphQL, OrderGraphQL, PaymentGraphQL, ProductGraphQL, 
    CartItemGraphQL, ReviewGraphQL, CartItemWithProductGraphQL,
    OrderItemGraphQL, CategoryGraphQL, ProductDetailGraphQL,
    ReviewDetailGraphQL, UserDetailGraphQL, ProductWithRatingGraphQL,
    CategoryWithProductsGraphQL, OrderWithItemsGraphQL, ProductSummaryGraphQL
};

#[derive(Default)]
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

    /// Get user detail by ID (extended info)
    async fn user_detail(&self, ctx: &Context<'_>, id: i64) -> Result<Option<UserDetailGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_id(db, id).await?;
        Ok(user.map(UserDetailGraphQL::from))
    }

    /// Get user by email
    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_email(db, &email).await?;
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

    /// Get product detail by ID (extended info)
    async fn product_detail(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ProductDetailGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let product = ProductService::get_by_id(db, id).await?;
        Ok(product.map(ProductDetailGraphQL::from))
    }

    /// Get product summary by ID (minimal info for listings)
    async fn product_summary(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ProductSummaryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let product = ProductService::get_by_id(db, id).await?;
        Ok(product.map(ProductSummaryGraphQL::from))
    }

    /// Get product with rating info
    async fn product_with_rating(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ProductWithRatingGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let product = ProductService::get_by_id(db, id).await?;
        
        if let Some(product) = product {
            let average_rating = ReviewService::get_average_rating(db, id).await?;
            let review_count = ReviewService::get_review_count(db, id).await?;
            Ok(Some(ProductWithRatingGraphQL::from_product_with_rating(
                product,
                average_rating,
                review_count,
            )))
        } else {
            Ok(None)
        }
    }

    /// Get all products with ratings
    async fn products_with_ratings(&self, ctx: &Context<'_>) -> Result<Vec<ProductWithRatingGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = ProductService::get_all(db).await?;
        
        let mut result = Vec::new();
        for product in products {
            let average_rating = ReviewService::get_average_rating(db, product.id).await?;
            let review_count = ReviewService::get_review_count(db, product.id).await?;
            result.push(ProductWithRatingGraphQL::from_product_with_rating(
                product,
                average_rating,
                review_count,
            ));
        }
        
        Ok(result)
    }

    /// Get product summaries (for listing pages)
    async fn product_summaries(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = true)] active_only: bool
    ) -> Result<Vec<ProductSummaryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let products = if active_only {
            ProductService::get_active(db).await?
        } else {
            ProductService::get_all(db).await?
        };
        Ok(products.into_iter().map(ProductSummaryGraphQL::from).collect())
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

    // ===================== CATEGORY QUERIES ====================
    
    /// Get all categories
    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let categories = crate::services::CategoryService::get_all_categories(db).await?;
        Ok(categories.into_iter().map(CategoryGraphQL::from).collect())
    }
    
    /// Get category by ID
    async fn category(&self, ctx: &Context<'_>, id: i32) -> Result<Option<CategoryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let category = crate::services::CategoryService::get_category_by_id(db, id).await?;
        Ok(category.map(CategoryGraphQL::from))
    }

    /// Get category with products
    async fn category_with_products(&self, ctx: &Context<'_>, id: i32) -> Result<Option<CategoryWithProductsGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let category = crate::services::CategoryService::get_category_by_id(db, id).await?;
        
        if let Some(category) = category {
            let products = ProductService::get_by_category(db, id as i64).await?;
            let mut result = CategoryWithProductsGraphQL::from(category);
            result.products = products.into_iter().map(ProductGraphQL::from).collect();
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Get all categories with their products
    async fn categories_with_products(&self, ctx: &Context<'_>) -> Result<Vec<CategoryWithProductsGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let categories = crate::services::CategoryService::get_all_categories(db).await?;
        
        let mut result = Vec::new();
        for category in categories {
            let products = ProductService::get_by_category(db, category.id).await?;
            let mut cat_with_products = CategoryWithProductsGraphQL::from(category);
            cat_with_products.products = products.into_iter().map(ProductGraphQL::from).collect();
            result.push(cat_with_products);
        }
        
        Ok(result)
    }

    // ==================== CART QUERIES ====================

    /// Get cart items for user
    async fn cart_items(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<CartItemGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cart_items = CartService::get_cart_items(db, user_id).await?;
        Ok(cart_items.into_iter().map(CartItemGraphQL::from).collect())
    }

    /// Get cart items with product details
    async fn cart_items_with_products(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<CartItemWithProductGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cart_items = CartService::get_cart_items(db, user_id).await?;
        
        let mut result = Vec::new();
        for cart_item in cart_items {
            if let Some(product) = ProductService::get_by_id(db, cart_item.product_id.unwrap_or(0)).await? {
                result.push(CartItemWithProductGraphQL {
                    id: cart_item.id,
                    cart_id: cart_item.cart_id,
                    product_id: cart_item.product_id.unwrap_or(0),
                    quantity: cart_item.quantity,
                    product: ProductGraphQL::from(product),
                });
            }
        }
        
        Ok(result)
    }

    /// Get cart item count for user
    async fn cart_count(&self, ctx: &Context<'_>, user_id: i64) -> Result<usize> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = CartService::get_cart_count(db, user_id).await?;
        Ok(count)
    }

    /// Get cart total amount for user
    async fn cart_total(&self, ctx: &Context<'_>, user_id: i64) -> Result<f64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let cart_items = CartService::get_cart_items(db, user_id).await?;
        
        let mut total = 0.0;
        for cart_item in cart_items {
            if let Some(product) = ProductService::get_by_id(db, cart_item.product_id.unwrap_or(0)).await? {
                let price = product.price.to_f64().unwrap_or(0.0);
                total += price * cart_item.quantity as f64;
            }
        }
        
        Ok(total)
    }

    // ==================== REVIEW QUERIES ====================

    /// Get review by ID
    async fn review(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let review = ReviewService::get_by_id(db, id).await?;
        Ok(review.map(ReviewGraphQL::from))
    }

    /// Get review detail by ID
    async fn review_detail(&self, ctx: &Context<'_>, id: i64) -> Result<Option<ReviewDetailGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let review = ReviewService::get_by_id(db, id).await?;
        Ok(review.map(ReviewDetailGraphQL::from))
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

    /// Get order with items by ID
    async fn order_with_items(&self, ctx: &Context<'_>, id: i64) -> Result<Option<OrderWithItemsGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let order = OrderService::get_by_id(db, id).await?;
        
        if let Some(order) = order {
            let order_items = crate::services::OrderItemService::get_by_order(db, id).await?;
            Ok(Some(OrderWithItemsGraphQL {
                id: order.id,
                user_id: order.user_id,
                total_amount: order.total_price.to_f64().unwrap_or(0.0),
                status: order.status,
                created_at: order.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                updated_at: order.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                items: order_items.into_iter().map(OrderItemGraphQL::from).collect(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get orders by user ID
    async fn user_orders(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let orders = OrderService::get_user_orders(db, user_id).await?;
        Ok(orders.into_iter().map(OrderGraphQL::from).collect())
    }

    /// Get orders with items by user ID
    async fn user_orders_with_items(&self, ctx: &Context<'_>, user_id: i64) -> Result<Vec<OrderWithItemsGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let orders = OrderService::get_user_orders(db, user_id).await?;
        
        let mut result = Vec::new();
        for order in orders {
            let order_items = crate::services::OrderItemService::get_by_order(db, order.id).await?;
            result.push(OrderWithItemsGraphQL {
                id: order.id,
                user_id: order.user_id,
                total_amount: order.total_price.to_f64().unwrap_or(0.0),
                status: order.status,
                created_at: order.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                updated_at: order.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                items: order_items.into_iter().map(OrderItemGraphQL::from).collect(),
            });
        }
        
        Ok(result)
    }

    /// Get order items by order ID
    async fn order_items(&self, ctx: &Context<'_>, order_id: i64) -> Result<Vec<OrderItemGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let order_items = crate::services::OrderItemService::get_by_order(db, order_id).await?;
        Ok(order_items.into_iter().map(OrderItemGraphQL::from).collect())
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