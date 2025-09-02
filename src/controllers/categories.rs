use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use sqlx::SqlitePool;

pub async fn list(
    _pool: &SqlitePool,
    page: i32,
    per_page: i32,
    _offset: i32,
    _sort: Option<String>,
    _filter: Option<String>,
    _expand: Option<String>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "page": page,
        "perPage": per_page,
        "totalItems": 0,
        "totalPages": 0,
        "items": []
    })))
}

pub async fn get(
    _pool: &SqlitePool,
    id: &str,
    _expand: Option<String>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "id": id,
        "name": "",
        "slug": ""
    })))
}

pub async fn create(_pool: &SqlitePool, _data: Value) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "id": "new_category_id",
        "created": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn update(_pool: &SqlitePool, id: &str, _data: Value) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "id": id,
        "updated": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn delete(_pool: &SqlitePool, _id: &str) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}