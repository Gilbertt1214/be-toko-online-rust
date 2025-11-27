use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::ReviewGraphQL;
use crate::services::ReviewService;

#[derive(Default)]
pub struct ReviewQueries;

#[Object]
impl ReviewQueries {
    /// Get all reviews for a product
    async fn reviews_by_product(&self, ctx: &Context<'_>, product_id: i64) -> Result<Vec<ReviewGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let reviews = ReviewService::get_by_product(db, product_id)
            .await
            .map_err(Error::new)?;

        Ok(reviews.into_iter().map(ReviewGraphQL::from).collect())
    }

    /// Get average rating for a product
    async fn product_rating(&self, ctx: &Context<'_>, product_id: i64) -> Result<f64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let rating = ReviewService::get_average_rating(db, product_id)
            .await
            .map_err(Error::new)?;

        Ok(rating)
    }

    /// Get review count for a product
    async fn review_count(&self, ctx: &Context<'_>, product_id: i64) -> Result<i64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = ReviewService::get_review_count(db, product_id)
            .await
            .map_err(Error::new)?;

        Ok(count as i64)
    }
}
