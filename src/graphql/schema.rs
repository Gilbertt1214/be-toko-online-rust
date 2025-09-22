use async_graphql::{Schema};
use super::{query::QueryRoot, mutation::MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription).finish()
}
