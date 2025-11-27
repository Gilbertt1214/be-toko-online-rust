use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::{
    ProductGraphQL, ProductDetailGraphQL, ProductSummaryGraphQL, 
    ProductWithRatingGraphQL, CategoryGraphQL
};
use crate::services::{ProductService, ReviewService, CategoryService};

#[derive(Default)]
pub struct ProductQueries;

#[Object]
impl ProductQueries {
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

    /// Get all categories
    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let categories = CategoryService::get_all(db).await?;
        Ok(categories.into_iter().map(CategoryGraphQL::from).collect())
    }

    /// Get category by ID
    async fn category(&self, ctx: &Context<'_>, id: i32) -> Result<Option<CategoryGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let category = CategoryService::get_by_id(db, id).await?;
        Ok(category.map(CategoryGraphQL::from))
    }
}
