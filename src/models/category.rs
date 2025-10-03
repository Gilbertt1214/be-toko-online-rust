use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::{SimpleObject};
use chrono::NaiveDateTime;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel,
    Serialize, Deserialize, SimpleObject
)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub slug: String,

    #[graphql(skip)]
    pub created_at: Option<NaiveDateTime>,

    #[graphql(skip)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product::Entity")]
    Product,
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
