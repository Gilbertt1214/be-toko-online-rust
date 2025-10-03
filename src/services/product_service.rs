use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::models::{product, prelude::Product};

pub struct ProductService;

impl ProductService {
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        slug: String,
        description: Option<String>,
        price_str: String,
        stock: i32,
        category_id: Option<i64>,
        seller_id: Option<i64>,
    ) -> Result<product::Model, String> {
        // Parse price
        let price = BigDecimal::from_str(&price_str)
            .map_err(|_| "Format harga tidak valid".to_string())?;

        // Validasi slug
        let existing = Product::find()
            .filter(product::Column::Slug.eq(&slug))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing.is_some() {
            return Err("Slug sudah digunakan".to_string());
        }

        // Insert
        let new_product = product::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            description: Set(description),
            price: Set(price),
            stock: Set(stock),
            category_id: Set(category_id),
            seller_id: Set(seller_id),
            is_active: Set(Some(true)),
            ..Default::default()
        };

        new_product
            .insert(db)
            .await
            .map_err(|e| format!("Gagal membuat produk: {}", e))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        slug: Option<String>,
        description: Option<String>,
        price: Option<String>,
        stock: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Option<product::Model>, String> {
        let Some(existing) = Product::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        let mut active: product::ActiveModel = existing.into();

        if let Some(n) = name {
            active.name = Set(n);
        }

        if let Some(s) = slug {
            let slug_exists = Product::find()
                .filter(product::Column::Slug.eq(&s))
                .filter(product::Column::Id.ne(id))
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            if slug_exists.is_some() {
                return Err("Slug sudah digunakan".to_string());
            }
            active.slug = Set(s);
        }

        if let Some(d) = description {
            active.description = Set(Some(d));
        }

        if let Some(p) = price {
            let price_decimal = BigDecimal::from_str(&p)
                .map_err(|_| "Format harga tidak valid".to_string())?;
            active.price = Set(price_decimal);
        }

        if let Some(s) = stock {
            active.stock = Set(s);
        }

        if let Some(flag) = is_active {
            active.is_active = Set(Some(flag));
        }

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Gagal update produk: {}", e))
    }

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
        active
            .update(db)
            .await
            .map_err(|e| format!("Gagal soft delete: {}", e))?;

        Ok(true)
    }

    pub async fn hard_delete(db: &DatabaseConnection, id: i64) -> Result<bool, String> {
        let res = Product::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        Ok(res.rows_affected > 0)
    }
    // Tambahkan method ini di impl ProductService

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<product::Model>, String> {
    Product::find()
        .all(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> Result<Option<product::Model>, String> {
    Product::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn get_product_by_slug(
    db: &DatabaseConnection,
    slug: &str,
) -> Result<Option<product::Model>, String> {
    Product::find()
        .filter(product::Column::Slug.eq(slug))
        .one(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn search_products(
    db: &DatabaseConnection,
    keyword: &str,
) -> Result<Vec<product::Model>, String> {
    use sea_orm::sea_query::Expr;
    
    Product::find()
        .filter(
            Expr::col(product::Column::Name)
                .like(format!("%{}%", keyword))
        )
        .all(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}
}