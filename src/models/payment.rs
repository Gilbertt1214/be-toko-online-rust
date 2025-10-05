use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order_id: i64,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub paid_at: Option<NaiveDateTime>,
    
    // Xendit Integration Fields
    pub external_id: Option<String>,        // Order reference ID
    pub invoice_id: Option<String>,         // Xendit invoice ID
    pub invoice_url: Option<String>,        // Payment URL for customer
    pub xendit_status: Option<String>,      // PENDING, PAID, EXPIRED, SETTLED
    pub payment_channel: Option<String>,    // BCA, OVO, DANA, etc
    pub paid_amount: Option<f64>,           // Actual amount paid
    pub payment_method: Option<String>,     // BANK_TRANSFER, EWALLET, CREDIT_CARD
    pub xendit_fees: Option<f64>,           // Transaction fees
    pub expiry_date: Option<NaiveDateTime>, // Invoice expiration
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::order::Entity",
        from = "Column::OrderId",
        to = "super::order::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Order,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}