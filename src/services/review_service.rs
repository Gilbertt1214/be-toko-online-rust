use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::models::{prelude::Review, review};

#[allow(dead_code)]
pub struct ReviewService;

impl ReviewService {
    /// Create new review
    #[allow(dead_code)]
    pub async fn create_review(
        db: &DatabaseConnection,
        user_id: i64,
        product_id: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<review::Model, String> {
        if !(1..=5).contains(&rating) {
            return Err("Rating must be between 1-5".to_string());
        }

        let new_review = review::ActiveModel {
            user_id: Set(user_id),
            product_id: Set(Some(product_id)),
            rating: Set(rating),
            comment: Set(comment),
            ..Default::default()
        };

        new_review
            .insert(db)
            .await
            .map_err(|e| format!("Failed to create review: {}", e))
    }

    /// Get review by ID
    #[allow(dead_code)]
    pub async fn get_by_id(
        db: &DatabaseConnection,
        review_id: i64,
    ) -> Result<Option<review::Model>, String> {
        Review::find_by_id(review_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all reviews for a product
    pub async fn get_by_product(
        db: &DatabaseConnection,
        product_id: i64,
    ) -> Result<Vec<review::Model>, String> {
        Review::find()
            .filter(review::Column::ProductId.eq(product_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get all reviews by a user
    #[allow(dead_code)]
    pub async fn get_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<review::Model>, String> {
        Review::find()
            .filter(review::Column::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get user's review for a specific product
    #[allow(dead_code)]
    pub async fn get_user_review(
        db: &DatabaseConnection,
        user_id: i64,
        product_id: i64,
    ) -> Result<Option<review::Model>, String> {
        Review::find()
            .filter(review::Column::UserId.eq(user_id))
            .filter(review::Column::ProductId.eq(product_id))
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Update review
    #[allow(dead_code)]
    pub async fn update_review(
        db: &DatabaseConnection,
        review_id: i64,
        rating: Option<i32>,
        comment: Option<String>,
    ) -> Result<Option<review::Model>, String> {
        let Some(existing) = Review::find_by_id(review_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
        else {
            return Ok(None);
        };

        let mut active: review::ActiveModel = existing.into();

        if let Some(r) = rating {
            if !(1..=5).contains(&r) {
                return Err("Rating must be between 1-5".to_string());
            }
            active.rating = Set(r);
        }

        if let Some(c) = comment {
            active.comment = Set(Some(c));
        }

        active
            .update(db)
            .await
            .map(Some)
            .map_err(|e| format!("Failed to update review: {}", e))
    }

    /// Delete review
    #[allow(dead_code)]
    pub async fn delete_review(db: &DatabaseConnection, review_id: i64) -> Result<bool, String> {
        let res = Review::delete_by_id(review_id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Calculate average rating for a product
    pub async fn get_average_rating(
        db: &DatabaseConnection,
        product_id: i64,
    ) -> Result<f64, String> {
        let reviews = Self::get_by_product(db, product_id).await?;

        if reviews.is_empty() {
            return Ok(0.0);
        }

        let total: i32 = reviews.iter().map(|r| r.rating).sum();
        let average = total as f64 / reviews.len() as f64;

        Ok(average)
    }

    /// Get review count for a product
    pub async fn get_review_count(
        db: &DatabaseConnection,
        product_id: i64,
    ) -> Result<usize, String> {
        let reviews = Self::get_by_product(db, product_id).await?;
        Ok(reviews.len())
    }
}