use sea_orm::entity::prelude::*;
use async_graphql::{SimpleObject, Enum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: UserRole,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(
    Copy,
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
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
