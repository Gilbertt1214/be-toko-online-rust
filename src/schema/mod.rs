pub mod mutations;
pub mod queries;

pub use mutations::MutationRoot;
pub use queries::QueryRoot;

use async_graphql::*;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[allow(dead_code)]
pub fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish()
}