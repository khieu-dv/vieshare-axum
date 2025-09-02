use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use sqlx::SqlitePool;
use std::collections::HashMap;

use super::{users, categories, stores};

// Query parameters for list operations
#[derive(serde::Deserialize)]
pub struct ListQuery {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub sort: Option<String>,
    pub filter: Option<String>,
    pub expand: Option<String>,
}

// Health check endpoint
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// List records for a collection
pub async fn list_records(
    State(pool): State<SqlitePool>,
    Path(collection): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Value>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(30);
    let offset = (page - 1) * per_page;

    match collection.as_str() {
        "users" => users::list(&pool, page, per_page, offset, query.sort, query.filter, query.expand).await,
        "categories" => categories::list(&pool, page, per_page, offset, query.sort, query.filter, query.expand).await,
        "stores" => stores::list(&pool, page, per_page, offset, query.sort, query.filter, query.expand).await,
        _ => Err(StatusCode::NOT_FOUND),
    }
}

// Get a specific record
pub async fn get_record(
    State(pool): State<SqlitePool>,
    Path((collection, id)): Path<(String, String)>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    let expand = query.get("expand").cloned();

    match collection.as_str() {
        "users" => users::get(&pool, &id, expand).await,
        "categories" => categories::get(&pool, &id, expand).await,
        "stores" => stores::get(&pool, &id, expand).await,
        _ => Err(StatusCode::NOT_FOUND),
    }
}

// Create a new record
pub async fn create_record(
    State(pool): State<SqlitePool>,
    Path(collection): Path<String>,
    Json(data): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    match collection.as_str() {
        "users" => users::create(&pool, data).await,
        "categories" => categories::create(&pool, data).await,
        "stores" => stores::create(&pool, data).await,
        _ => Err(StatusCode::NOT_FOUND),
    }
}

// Update an existing record
pub async fn update_record(
    State(pool): State<SqlitePool>,
    Path((collection, id)): Path<(String, String)>,
    Json(data): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    match collection.as_str() {
        "users" => users::update(&pool, &id, data).await,
        "categories" => categories::update(&pool, &id, data).await,
        "stores" => stores::update(&pool, &id, data).await,
        _ => Err(StatusCode::NOT_FOUND),
    }
}

// Delete a record
pub async fn delete_record(
    State(pool): State<SqlitePool>,
    Path((collection, id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    match collection.as_str() {
        "users" => users::delete(&pool, &id).await,
        "categories" => categories::delete(&pool, &id).await,
        "stores" => stores::delete(&pool, &id).await,
        _ => Err(StatusCode::NOT_FOUND),
    }
}