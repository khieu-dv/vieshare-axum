use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::SqlitePool;

use crate::controllers::pocketbase::{
    health, list_records, get_record, create_record, update_record, delete_record
};

pub fn create_route(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/collections/:collection/records", get(list_records).post(create_record))
        .route("/api/collections/:collection/records/:id", get(get_record).patch(update_record).delete(delete_record))
        .with_state(pool)
}