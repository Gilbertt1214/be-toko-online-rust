use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set, 
};

use crate::models::{prelude::*, category};

pub struct CategoryService;

impl CategoryService {
    /// Create new category
    pub async fn create_category(
        db: &DatabaseConnection,
        name: String,
        slug: String,
    ) -> Result<category::Model, String> {
        if name.trim().is_empty() {
            return Err("Category name cannot be empty".to_string());
        }

        if slug.trim().is_empty() {
            return Err("Slug cannot be empty".to_string());
        }

        // Check if slug already exists
        let existing_slug = Category::find()
            .filter(category::Column::Slug.eq(&slug))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_slug.is_some() {
            return Err("Slug already exists".to_string());
        }

        let new_category = category::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            ..Default::default()
        };

        new_category
            .insert(db)
            .await
            .map_err(|e| format!("Failed to create category: {}", e))
    }

    /// Get category by ID
    pub async fn get_category_by_id(
        db: &DatabaseConnection,
        category_id: i32,
    ) -> Result<Option<category::Model>, String> {
        Category::find_by_id(category_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all categories
    pub async fn get_all_categories(
        db: &DatabaseConnection,
    ) -> Result<Vec<category::Model>, String> {
        Category::find()
            .order_by_asc(category::Column::Name)
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Update category
    pub async fn update_category(
        db: &DatabaseConnection,
        category_id: i32,
        name: Option<String>,
        slug: Option<String>,
    ) -> Result<category::Model, String> {
        let existing = Category::find_by_id(category_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Category not found".to_string())?;

        let mut active: category::ActiveModel = existing.into();

        if let Some(n) = name {
            if n.trim().is_empty() {
                return Err("Category name cannot be empty".to_string());
            }
            active.name = Set(n);
        }

        if let Some(s) = slug {
            if s.trim().is_empty() {
                return Err("Slug cannot be empty".to_string());
            }

            // Check if new slug already exists
            let slug_exists = Category::find()
                .filter(category::Column::Slug.eq(&s))
                .filter(category::Column::Id.ne(category_id))
                .one(db)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            if slug_exists.is_some() {
                return Err("Slug already exists".to_string());
            }

            active.slug = Set(s);
        }

        active
            .update(db)
            .await
            .map_err(|e| format!("Failed to update category: {}", e))
    }

    /// Delete category
  pub async fn delete_category(
    db: &DatabaseConnection,
    category_id: i32,
) -> Result<bool, String> {
    use sea_orm::EntityTrait;
    
    // Check if category has products - FIX: gunakan PaginatorTrait
    use sea_orm::PaginatorTrait;
    
    let product_count = Product::find()
        .filter(crate::models::product::Column::CategoryId.eq(category_id as i64))
        .paginate(db, 1)
        .num_items()
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if product_count > 0 {
        return Err(format!(
            "Cannot delete category with {} products",
            product_count
        ));
    }

    let res = Category::delete_by_id(category_id)
        .exec(db)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(res.rows_affected > 0)
}
}