use async_graphql::*;
use sea_orm::DatabaseConnection;
use crate::graphql::graphql_types::{UserGraphQL, UserDetailGraphQL};
use crate::services::UserService;

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    /// Get all users
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let users = UserService::get_all(db).await?;
        Ok(users.into_iter().map(UserGraphQL::from).collect())
    }

    /// Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: i64) -> Result<Option<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_id(db, id).await?;
        Ok(user.map(UserGraphQL::from))
    }

    /// Get user detail by ID (extended info)
    async fn user_detail(&self, ctx: &Context<'_>, id: i64) -> Result<Option<UserDetailGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_id(db, id).await?;
        Ok(user.map(UserDetailGraphQL::from))
    }

    /// Get user by email
    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<UserGraphQL>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = UserService::get_by_email(db, &email).await?;
        Ok(user.map(UserGraphQL::from))
    }
}
