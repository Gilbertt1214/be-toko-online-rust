use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "carts")]
#[graphql(name = "Cart")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::cart_item::Entity")]
    CartItem,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::cart_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CartItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}