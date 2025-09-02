use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseRecord {
    pub id: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "collectionName")]
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct User {
    pub id: String,
    #[validate(email)]
    pub email: String,
    pub email_visibility: bool,
    #[validate(length(min = 1))]
    pub username: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub verified: bool,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Category {
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub slug: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Subcategory {
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub slug: String,
    pub description: Option<String>,
    pub category: String,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Store {
    pub id: String,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(length(min = 1))]
    pub slug: String,
    pub description: Option<String>,
    pub user: String,
    pub plan: String,
    pub plan_ends_at: Option<String>,
    pub cancel_plan_at_end: bool,
    pub product_limit: i32,
    pub tag_limit: i32,
    pub variant_limit: i32,
    pub active: bool,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub images: Option<String>, // JSON array as string
    pub category: String,
    pub subcategory: Option<String>,
    #[validate(length(min = 1))]
    pub price: String,
    pub inventory: i32,
    pub rating: f64,
    pub store: String,
    pub active: bool,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cart {
    pub id: String,
    pub user: Option<String>,
    pub session_id: Option<String>,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct CartItem {
    pub id: String,
    pub cart: String,
    pub product: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
    pub subcategory: Option<String>,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Address {
    pub id: String,
    #[validate(length(min = 1))]
    pub line1: String,
    pub line2: Option<String>,
    #[validate(length(min = 1))]
    pub city: String,
    #[validate(length(min = 1))]
    pub state: String,
    #[validate(length(min = 1))]
    pub postal_code: String,
    #[validate(length(min = 1))]
    pub country: String,
    pub user: String,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Order {
    pub id: String,
    pub user: Option<String>,
    pub store: String,
    pub items: String, // JSON as string
    pub quantity: Option<i32>,
    #[validate(length(min = 1))]
    pub amount: String,
    pub status: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub address: String,
    pub notes: Option<String>,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub name: Option<String>,
    #[validate(email)]
    pub email: String,
    pub store: String,
    pub total_orders: i32,
    pub total_spent: String,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Notification {
    pub id: String,
    #[validate(email)]
    pub email: String,
    pub token: String,
    pub user: Option<String>,
    pub communication: bool,
    pub newsletter: bool,
    pub marketing: bool,
    pub created: String,
    pub updated: String,
    pub collection_id: String,
    pub collection_name: String,
}

// PocketBase API Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct PBListResponse<T> {
    pub page: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "totalItems")]
    pub total_items: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBAuthResponse<T> {
    pub token: String,
    pub record: T,
}

// Expand structures for relations
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandData {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

// Helper functions for PocketBase compatibility
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)[..15].to_string()
}

pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S%.3fZ").to_string()
}

pub fn calculate_total_pages(total_items: i32, per_page: i32) -> i32 {
    if total_items == 0 {
        0
    } else {
        (total_items + per_page - 1) / per_page
    }
}

// Common traits and implementations
impl User {
    pub fn new(email: String, username: String, name: Option<String>) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            email,
            email_visibility: false,
            username,
            name,
            avatar: None,
            verified: false,
            created: now.clone(),
            updated: now,
            collection_id: "users".to_string(),
            collection_name: "users".to_string(),
        }
    }
}

impl Category {
    pub fn new(name: String, slug: String, description: Option<String>, image: Option<String>) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name,
            slug,
            description,
            image,
            created: now.clone(),
            updated: now,
            collection_id: "categories".to_string(),
            collection_name: "categories".to_string(),
        }
    }
}

impl Store {
    pub fn new(name: String, slug: String, user: String) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name,
            slug,
            description: None,
            user,
            plan: "free".to_string(),
            plan_ends_at: None,
            cancel_plan_at_end: false,
            product_limit: 10,
            tag_limit: 5,
            variant_limit: 5,
            active: true,
            created: now.clone(),
            updated: now,
            collection_id: "stores".to_string(),
            collection_name: "stores".to_string(),
        }
    }
}

impl Product {
    pub fn new(name: String, price: String, category: String, store: String) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name,
            description: None,
            images: None,
            category,
            subcategory: None,
            price,
            inventory: 0,
            rating: 0.0,
            store,
            active: true,
            created: now.clone(),
            updated: now,
            collection_id: "products".to_string(),
            collection_name: "products".to_string(),
        }
    }
}

impl Cart {
    pub fn new(user: Option<String>, session_id: Option<String>) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            user,
            session_id,
            created: now.clone(),
            updated: now,
            collection_id: "carts".to_string(),
            collection_name: "carts".to_string(),
        }
    }
}

impl Address {
    pub fn new(line1: String, city: String, state: String, postal_code: String, country: String, user: String) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            line1,
            line2: None,
            city,
            state,
            postal_code,
            country,
            user,
            created: now.clone(),
            updated: now,
            collection_id: "addresses".to_string(),
            collection_name: "addresses".to_string(),
        }
    }
}