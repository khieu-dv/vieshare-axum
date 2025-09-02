use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSessionInfo {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataList<T> {
    pub data: T,
    pub meta: Option<serde_json::Value>,
}

impl<T> DataList<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            meta: None,
        }
    }

    pub fn with_meta(data: T, meta: serde_json::Value) -> Self {
        Self {
            data,
            meta: Some(meta),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(page: i32, per_page: i32, total_items: i64) -> Self {
        let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;
        Self {
            page,
            per_page,
            total_items,
            total_pages,
        }
    }
}

pub fn calculate_offset(page: i32, per_page: i32) -> i32 {
    (page - 1) * per_page
}