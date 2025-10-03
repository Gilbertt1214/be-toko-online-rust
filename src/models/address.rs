use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "addresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub user_id: Option<i64>,
    pub label: Option<String>,
    pub recipient_name: Option<String>,
    pub phone: Option<String>,
    pub address_line: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
