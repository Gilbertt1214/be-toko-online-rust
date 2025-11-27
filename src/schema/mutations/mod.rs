mod auth;
mod user;
mod product;
mod cart;
mod order;
mod payment;

pub use auth::AuthMutations;
pub use user::UserMutations;
pub use product::ProductMutations;
pub use cart::CartMutations;
pub use order::OrderMutations;
pub use payment::PaymentMutations;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    AuthMutations,
    UserMutations,
    ProductMutations,
    CartMutations,
    OrderMutations,
    PaymentMutations,
);
