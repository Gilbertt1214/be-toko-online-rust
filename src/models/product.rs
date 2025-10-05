use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "products")]
#[graphql(name = "Product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub category_id: Option<i64>,
    pub seller_id: Option<i64>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SellerId",
        to = "super::user::Column::Id"
    )]
    Seller,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}