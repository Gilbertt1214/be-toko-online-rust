use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::models::{product, prelude::*};
use crate::models::user::UserRole;
use crate::services::ProductService;
use crate::utils::jwt;

#[derive(Default)]
pub struct ProductMutations;

#[Object]
impl ProductMutations {
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

        let existing = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        if claims.role == UserRole::Pengusaha {
            if existing.seller_id != Some(claims.user_id) {
                return Err(Error::new("Unauthorized: You can only update your own products"));
            }
        }

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

    /// Update product stock (Admin/Pengusaha only)
    async fn update_product_stock(
        &self,
        ctx: &Context<'_>,
        token: String,
        product_id: i64,
        stock: i32,
    ) -> Result<product::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin && claims.role != UserRole::Pengusaha {
            return Err(Error::new("Unauthorized"));
        }

        if stock < 0 {
            return Err(Error::new("Stock tidak boleh negatif"));
        }

        let product = Product::find_by_id(product_id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("Product not found"))?;

        if claims.role == UserRole::Pengusaha && product.seller_id != Some(claims.user_id) {
            return Err(Error::new("Unauthorized"));
        }

        let mut product_active: product::ActiveModel = product.into();
        product_active.stock = Set(stock);

        product_active
            .update(db)
            .await
            .map_err(|e| Error::new(format!("Failed to update stock: {}", e)))
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

        let existing = ProductService::get_by_id(db, product_id)
            .await
            .map_err(Error::new)?
            .ok_or_else(|| Error::new("Product not found"))?;

        if claims.role == UserRole::Pengusaha {
            if existing.seller_id != Some(claims.user_id) {
                return Err(Error::new("Unauthorized: You can only delete your own products"));
            }
        }

        let success = if hard_delete.unwrap_or(false) {
            if claims.role != UserRole::Admin {
                return Err(Error::new("Unauthorized: Only Admin can permanently delete products"));
            }
            ProductService::hard_delete(db, product_id)
                .await
                .map_err(Error::new)?
        } else {
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
}
