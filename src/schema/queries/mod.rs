mod user;
mod product;
mod cart;
mod order;
mod review;

pub use user::UserQueries;
pub use product::ProductQueries;
pub use cart::CartQueries;
pub use order::OrderQueries;
pub use review::ReviewQueries;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    UserQueries,
    ProductQueries,
    CartQueries,
    OrderQueries,
    ReviewQueries,
);
