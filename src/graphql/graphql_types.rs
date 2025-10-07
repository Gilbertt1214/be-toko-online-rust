// src/graphql/graphql_types.rs
use async_graphql::SimpleObject;
use bigdecimal::ToPrimitive;
use std::fmt;
use crate::models::{user, product, order, order_item, cart_item, review};
use crate::models::user::UserRole;
use crate::services::payment_service::XenditInvoiceResponse;

// --- UserRole Display (fix) ---

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Pengguna => write!(f, "pengguna"),
            UserRole::Pengusaha => write!(f, "pengusaha"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

// --- GraphQL Types ---

// User GraphQL Type
#[derive(SimpleObject)]
pub struct UserGraphQL {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<user::Model> for UserGraphQL {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            role: model.role.to_string(),
        }
    }
}

// Product GraphQL Type (Basic)
#[derive(SimpleObject)]
pub struct ProductGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: Option<i64>,
    pub seller_id: Option<i64>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<product::Model> for ProductGraphQL {
    fn from(model: product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            short_description: model.short_description,
            description: model.description,
            price: model.price.to_f64().unwrap_or(0.0),
            stock: model.stock,
            category_id: model.category_id,
            seller_id: model.seller_id,
            is_active: model.is_active,
            created_at: model.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: model.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// Order GraphQL Type
#[derive(SimpleObject)]
pub struct OrderGraphQL {
    pub id: i64,
    pub user_id: i64,
    pub total_amount: f64,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<order::Model> for OrderGraphQL {
    fn from(model: order::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            total_amount: model.total_price.to_f64().unwrap_or(0.0),
            status: model.status,
            created_at: model.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: model.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// Order Item GraphQL Type
#[derive(SimpleObject)]
pub struct OrderItemGraphQL {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub price: f64,
}

impl From<order_item::Model> for OrderItemGraphQL {
    fn from(model: order_item::Model) -> Self {
        Self {
            id: model.id,
            order_id: model.order_id,
            product_id: model.product_id.unwrap_or(0),
            quantity: model.quantity,
            price: model.price.to_f64().unwrap_or(0.0),
        }
    }
}

// Cart Item GraphQL Type
#[derive(SimpleObject)]
pub struct CartItemGraphQL {
    pub id: i64,
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

impl From<cart_item::Model> for CartItemGraphQL {
    fn from(model: cart_item::Model) -> Self {
        Self {
            id: model.id,
            cart_id: model.cart_id,
            product_id: model.product_id.unwrap_or(0),
            quantity: model.quantity,
        }
    }
}

// category GraphQL Type
#[derive(SimpleObject)]
pub struct CategoryGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
}
impl From<crate::models::category::Model> for CategoryGraphQL {
    fn from(model: crate::models::category::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
        }
    }
}


// Review GraphQL Type
#[derive(SimpleObject)]
pub struct ReviewGraphQL {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<review::Model> for ReviewGraphQL {
    fn from(model: review::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            product_id: model.product_id.unwrap_or(0),
            rating: model.rating,
            comment: model.comment,
            created_at: model.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: model.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// Payment GraphQL Type
#[derive(SimpleObject)]
pub struct PaymentGraphQL {
    pub id: String,
    pub external_id: String,
    pub invoice_url: String,
    pub status: String,
    pub expiry_date: String,
    pub amount: i64,
    pub paid_amount: i64,
    pub description: String,
}

impl From<XenditInvoiceResponse> for PaymentGraphQL {
    fn from(invoice: XenditInvoiceResponse) -> Self {
        Self {
            id: invoice.id,
            external_id: invoice.external_id,
            invoice_url: invoice.invoice_url,
            status: invoice.status,
            expiry_date: invoice.expiry_date,
            amount: invoice.amount,
            paid_amount: invoice.paid_amount,
            description: invoice.description,
        }
    }
}

// Extended Product GraphQL Type dengan informasi tambahan
#[derive(SimpleObject)]
pub struct ProductDetailGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub is_active: Option<bool>,
    pub category_id: Option<i64>,
    pub seller_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<product::Model> for ProductDetailGraphQL {
    fn from(model: product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            short_description: model.short_description,
            description: model.description,
            price: model.price.to_f64().unwrap_or(0.0),
            stock: model.stock,
            is_active: model.is_active,
            category_id: model.category_id,
            seller_id: model.seller_id,
            created_at: model.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: model.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// Extended Review GraphQL Type dengan informasi tambahan
#[derive(SimpleObject)]
pub struct ReviewDetailGraphQL {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<review::Model> for ReviewDetailGraphQL {
    fn from(model: review::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            product_id: model.product_id.unwrap_or(0),
            rating: model.rating,
            comment: model.comment,
            created_at: model.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: model.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// User dengan informasi tambahan
#[derive(SimpleObject)]
pub struct UserDetailGraphQL {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<user::Model> for UserDetailGraphQL {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            role: model.role.to_string(),
        }
    }
}

// Product dengan rating rata-rata
#[derive(SimpleObject)]
pub struct ProductWithRatingGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub is_active: Option<bool>,
    pub average_rating: f64,
    pub review_count: i64,
}

impl ProductWithRatingGraphQL {
    pub fn from_product_with_rating(
        product: product::Model,
        average_rating: f64,
        review_count: usize,
    ) -> Self {
        Self {
            id: product.id,
            name: product.name,
            slug: product.slug,
            short_description: product.short_description,
            description: product.description,
            price: product.price.to_f64().unwrap_or(0.0),
            stock: product.stock,
            is_active: product.is_active,
            average_rating,
            review_count: review_count as i64,
        }
    }
}

// Category
#[derive(SimpleObject)]
pub struct CategoryWithProductsGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub products: Vec<ProductGraphQL>,
}
impl From<crate::models::category::Model> for CategoryWithProductsGraphQL {
    fn from(model: crate::models::category::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            products: vec![], 
        }
    }
}

// Cart Item dengan Product Detail
#[derive(SimpleObject)]
pub struct CartItemWithProductGraphQL {
    pub id: i64,
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub product: ProductGraphQL,
}

// Order dengan Order Items
#[derive(SimpleObject)]
pub struct OrderWithItemsGraphQL {
    pub id: i64,
    pub user_id: i64,
    pub total_amount: f64,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub items: Vec<OrderItemGraphQL>,
}

// Product Summary (untuk listing/cards)
#[derive(SimpleObject)]
pub struct ProductSummaryGraphQL {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub is_active: Option<bool>,
}

impl From<product::Model> for ProductSummaryGraphQL {
    fn from(model: product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            short_description: model.short_description,
            price: model.price.to_f64().unwrap_or(0.0),
            stock: model.stock,
            is_active: model.is_active,
        }
    }
}
#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
    pub user: user::Model,
}

#[derive(SimpleObject)]
pub struct PaymentInvoiceResponse {
    pub invoice_id: String,
    pub external_id: String,
    pub invoice_url: String,
    pub amount: f64,
    pub status: String,
    pub expiry_date: String,
}

#[derive(SimpleObject)]
pub struct PaymentStatusResponse {
    pub invoice_id: String,
    pub external_id: String,
    pub status: String,
    pub amount: f64,
    pub paid_amount: f64,
}
#[derive(SimpleObject)]
pub struct OrderItemResponse {
    pub id: i64,
    pub order_id: i64,
    pub product_id: Option<i64>,
    pub price: f64,
    pub quantity: i32,
    pub subtotal: Option<f64>,
}