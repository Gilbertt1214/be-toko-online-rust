use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use std::str::FromStr;

use crate::models::{prelude::Product, product};

pub struct ProductService;

impl ProductService {
    /// Create new product
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        slug: String,
        short_description: Option<String>,
        description: Option<String>,
        price_str: String,
        stock: i32,
        category_id: Option<i64>,
        seller_id: Option<i64>,
    ) -> Result<product::Model, String> {
        // Validate price format
        let price = BigDecimal::from_str(&price_str)
            .map_err(|_| "Invalid price format".to_string())?;

        // Check if slug already exists
        let existing = Product::find()
            .filter(product::Column::Slug.eq(&slug))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing.is_some() {
            return Err("Slug already exists".to_string());
        }

        // Create new product with timestamps
        let now = chrono::Utc::now().naive_utc();
        
        let new_product = product::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            short_description: Set(short_description),
            description: Set(description),
            price: Set(price),
            stock: Set(stock),
            category_id: Set(category_id),
            seller_id: Set(seller_id),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            ..Default::default()
        };

        new_product
            .insert(db)
            .await
            .map_err(|e| format!("Failed to create product: {}", e))
    }

    /// Update product
    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        slug: Option<String>,
        short_description: Option<Option<String>>,
        description: Option<Option<String>>,
        price: Option<String>,
        stock: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Option<product::Model>, String> {
        // Find existing product
        let Some(existing) = Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        let mut active: product::ActiveModel = existing.into();

        // Update fields if provided
        if let Some(n) = name {
            active.name = Set(n);
        }

        if let Some(s) = slug {
            // Check if new slug already exists for other products
            let slug_exists = Product::find()
                .filter(product::Column::Slug.eq(&s))
                .filter(product::Column::Id.ne(id))
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            if slug_exists.is_some() {
                return Err("Slug already exists".to_string());
            }
            active.slug = Set(s);
        }

        if let Some(sd) = short_description {
            active.short_description = Set(sd);
        }

        if let Some(d) = description {
            active.description = Set(d);
        }

        if let Some(p) = price {
            let price_decimal = BigDecimal::from_str(&p)
                .map_err(|_| "Invalid price format".to_string())?;
            active.price = Set(price_decimal);
        }

        if let Some(s) = stock {
            active.stock = Set(s);
        }

        if let Some(flag) = is_active {
            active.is_active = Set(Some(flag));
        }

        // Always update timestamp
        active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Failed to update product: {}", e))
    }

    /// Soft delete product (set is_active to false)
    pub async fn soft_delete(db: &DatabaseConnection, id: i64) -> Result<bool, String> {
        let Some(existing) = Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(false);
        };

        let mut active: product::ActiveModel = existing.into();
        active.is_active = Set(Some(false));
        active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        
        active
            .update(db)
            .await
            .map_err(|e| format!("Failed to soft delete: {}", e))?;

        Ok(true)
    }

    /// Hard delete product (remove from database)
    pub async fn hard_delete(db: &DatabaseConnection, id: i64) -> Result<bool, String> {
        let res = Product::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Get product by ID
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Option<product::Model>, String> {
        Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all products (including inactive)
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<product::Model>, String> {
        Product::find()
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get product by slug
    pub async fn get_by_slug(
        db: &DatabaseConnection,
        slug: &str,
    ) -> Result<Option<product::Model>, String> {
        Product::find()
            .filter(product::Column::Slug.eq(slug))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Search products by name or description
    pub async fn search(
        db: &DatabaseConnection,
        keyword: &str,
    ) -> Result<Vec<product::Model>, String> {
        use sea_orm::sea_query::Expr;

        Product::find()
            .filter(
                Expr::col(product::Column::Name)
                    .like(format!("%{}%", keyword))
                    .or(Expr::col(product::Column::ShortDescription).like(format!("%{}%", keyword)))
                    .or(Expr::col(product::Column::Description).like(format!("%{}%", keyword)))
            )
            .filter(product::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products by category
    pub async fn get_by_category(
        db: &DatabaseConnection,
        category_id: i64,
    ) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::CategoryId.eq(category_id))
            .filter(product::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products by seller
    pub async fn get_by_seller(
        db: &DatabaseConnection,
        seller_id: i64,
    ) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::SellerId.eq(seller_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get active products only
    pub async fn get_active(db: &DatabaseConnection) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Reduce stock after purchase
    pub async fn reduce_stock(
        db: &DatabaseConnection,
        id: i64,
        quantity: i32,
    ) -> Result<Option<product::Model>, String> {
        let Some(existing) = Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        if existing.stock < quantity {
            return Err("Insufficient stock".to_string());
        }

        let mut active: product::ActiveModel = existing.into();
        let new_stock = active.stock.clone().unwrap() - quantity;
        active.stock = Set(new_stock);
        active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Failed to reduce stock: {}", e))
    }

    /// Increase stock (restock or cancel order)
    pub async fn increase_stock(
        db: &DatabaseConnection,
        id: i64,
        quantity: i32,
    ) -> Result<Option<product::Model>, String> {
        let Some(existing) = Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        let mut active: product::ActiveModel = existing.into();
        let new_stock = active.stock.clone().unwrap() + quantity;
        active.stock = Set(new_stock);
        active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Failed to increase stock: {}", e))
    }

    // ==================== NEW QUERY METHODS ====================

    /// Get products with pagination
    pub async fn get_paginated(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let offset = (page - 1) * page_size;
        
        let mut query = Product::find();
        
        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }
        
        query
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get total product count
    pub async fn count_all(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<u64, String> {
        use sea_orm::PaginatorTrait;
        
        let mut query = Product::find();
        
        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }
        
        query
            .paginate(db, 1)
            .num_items()
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products by price range
    pub async fn get_by_price_range(
        db: &DatabaseConnection,
        min_price: &str,
        max_price: &str,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let min = BigDecimal::from_str(min_price)
            .map_err(|_| "Invalid min price format".to_string())?;
        let max = BigDecimal::from_str(max_price)
            .map_err(|_| "Invalid max price format".to_string())?;

        let mut query = Product::find()
            .filter(product::Column::Price.gte(min))
            .filter(product::Column::Price.lte(max));

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products with low stock (below threshold)
    pub async fn get_low_stock(
        db: &DatabaseConnection,
        threshold: i32,
    ) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::Stock.lte(threshold))
            .filter(product::Column::Stock.gt(0))
            .filter(product::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get out of stock products
    pub async fn get_out_of_stock(db: &DatabaseConnection) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::Stock.eq(0))
            .filter(product::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get featured products (recent active products)
    pub async fn get_featured(
        db: &DatabaseConnection,
        limit: u64,
    ) -> Result<Vec<product::Model>, String> {
        Product::find()
            .filter(product::Column::IsActive.eq(true))
            .order_by_desc(product::Column::CreatedAt)
            .limit(limit)
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get recent products
    pub async fn get_recent(
        db: &DatabaseConnection,
        limit: u64,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_desc(product::Column::CreatedAt)
            .limit(limit);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products sorted by price (ascending)
    pub async fn get_sorted_by_price_asc(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_asc(product::Column::Price);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products sorted by price (descending)
    pub async fn get_sorted_by_price_desc(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_desc(product::Column::Price);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products sorted by name
    pub async fn get_sorted_by_name(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_asc(product::Column::Name);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products sorted by stock (ascending)
    pub async fn get_sorted_by_stock_asc(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_asc(product::Column::Stock);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get products sorted by stock (descending)
    pub async fn get_sorted_by_stock_desc(
        db: &DatabaseConnection,
        active_only: bool,
    ) -> Result<Vec<product::Model>, String> {
        let mut query = Product::find()
            .order_by_desc(product::Column::Stock);

        if active_only {
            query = query.filter(product::Column::IsActive.eq(true));
        }

        query
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }
}