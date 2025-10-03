use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::models::{review, prelude::Review};

pub struct ReviewService;

impl ReviewService {
    /// Buat review baru
    pub async fn create_review(
        db: &DatabaseConnection,
        user_id: i64,
        product_id: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<review::Model, String> {
        // Validasi rating (1-5)
        if !(1..=5).contains(&rating) {
            return Err("Rating harus antara 1-5".to_string());
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
            .map_err(|e| format!("Gagal membuat review: {}", e))
    }

    /// Ambil semua review berdasarkan product_id
    pub async fn get_reviews_by_product(
        db: &DatabaseConnection,
        product_id: i64,
    ) -> Result<Vec<review::Model>, String> {
        Review::find()
            .filter(review::Column::ProductId.eq(product_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Ambil review berdasarkan id
    pub async fn get_review_by_id(
        db: &DatabaseConnection,
        review_id: i64,
    ) -> Result<Option<review::Model>, String> {
        Review::find_by_id(review_id)
            .one(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Ambil semua review dari user tertentu
    pub async fn get_reviews_by_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<review::Model>, String> {
        Review::find()
            .filter(review::Column::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Update review
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
                return Err("Rating harus antara 1-5".to_string());
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
            .map_err(|e| format!("Gagal update review: {}", e))
    }

    /// Hapus review
    pub async fn delete_review(
        db: &DatabaseConnection,
        review_id: i64,
    ) -> Result<bool, String> {
        let res = Review::delete_by_id(review_id)
            .exec(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(res.rows_affected > 0)
    }

    /// Hitung rata-rata rating untuk produk
    pub async fn get_average_rating(
        db: &DatabaseConnection,
        product_id: i64,
    ) -> Result<f64, String> {
        let reviews = Self::get_reviews_by_product(db, product_id).await?;

        if reviews.is_empty() {
            return Ok(0.0);
        }

        let total: i32 = reviews.iter().map(|r| r.rating).sum();
        let average = total as f64 / reviews.len() as f64;

        Ok(average)
    }
}