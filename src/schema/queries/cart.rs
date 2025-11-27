use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::CartItemGraphQL;
use crate::services::CartService;
use crate::utils::jwt;

#[derive(Default)]
pub struct CartQueries;

#[Object]
impl CartQueries {
    /// Get current user's cart items
    async fn my_cart(&self, ctx: &Context<'_>, token: String) -> Result<Vec<CartItemGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let cart_items = CartService::get_cart_items(db, claims.user_id)
            .await
            .map_err(Error::new)?;

        Ok(cart_items.into_iter().map(CartItemGraphQL::from).collect())
    }

    /// Get cart item count for current user
    async fn cart_count(&self, ctx: &Context<'_>, token: String) -> Result<i64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let count = CartService::get_cart_count(db, claims.user_id)
            .await
            .map_err(Error::new)?;

        Ok(count as i64)
    }
}
