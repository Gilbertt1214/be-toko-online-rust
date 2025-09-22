use async_graphql::{Context, Object};
use sea_orm::{EntityTrait, DatabaseConnection};
use crate::entity::user;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<user::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let users = user::Entity::find().all(db).await?;
        Ok(users)
    }
}
