use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::OrderGraphQL;
use crate::services::OrderService;
use crate::utils::jwt;
use crate::models::user::UserRole;

#[derive(Default)]
pub struct OrderQueries;

#[Object]
impl OrderQueries {
    /// Get current user's orders
    async fn my_orders(&self, ctx: &Context<'_>, token: String) -> Result<Vec<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let orders = OrderService::get_user_orders(db, claims.user_id)
            .await
            .map_err(Error::new)?;

        Ok(orders.into_iter().map(OrderGraphQL::from).collect())
    }

    /// Get order by ID
    async fn order(&self, ctx: &Context<'_>, token: String, id: i64) -> Result<Option<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        let order = OrderService::get_by_id(db, id)
            .await
            .map_err(Error::new)?;

        if let Some(order_data) = order {
            if order_data.user_id != claims.user_id && claims.role != UserRole::Admin{
                return Err(Error::new("Unauthorized"));
            }
            Ok(Some(OrderGraphQL::from(order_data)))
        } else {
            Ok(None)
        }
    }

    /// Get all orders (Admin only)
    async fn all_orders(&self, ctx: &Context<'_>, token: String) -> Result<Vec<OrderGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let claims = jwt::verify_jwt(&token)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))?;

        if claims.role != UserRole::Admin {
            return Err(Error::new("Unauthorized: Admin only"));
        }

        let orders = OrderService::get_all_orders(db)
            .await
            .map_err(Error::new)?;

        Ok(orders.into_iter().map(OrderGraphQL::from).collect())
    }
}
