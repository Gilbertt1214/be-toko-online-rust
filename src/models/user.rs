use sea_orm::entity::prelude::*;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel,
    Serialize, Deserialize, SimpleObject
)]
#[sea_orm(table_name = "users")]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,  // Ubah ke i64 untuk konsistensi
    pub username: String,
    pub email: String,
    
    #[graphql(skip)]  // Jangan expose password di GraphQL
    pub password: Option<String>,
    
    pub role: UserRole,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cart::Entity")]
    Cart,
    
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
    
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
}

impl Related<super::cart::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cart.def()
    }
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, EnumIter,
    DeriveActiveEnum, Serialize, Deserialize,
    async_graphql::Enum
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "pengguna")]
    #[graphql(name = "PENGGUNA")]
    Pengguna,

    #[sea_orm(string_value = "pengusaha")]
    #[graphql(name = "PENGUSAHA")]
    Pengusaha,

    #[sea_orm(string_value = "admin")]
    #[graphql(name = "ADMIN")]
    Admin,
}

impl ActiveModelBehavior for ActiveModel {}