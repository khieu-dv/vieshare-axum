use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use std::fs::File;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "db/pocketbase_schema.db";
    let path = Path::new(db_path);
    
    // Create database file if it doesn't exist
    if !path.exists() {
        println!("Creating new database file: {}", db_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        File::create(db_path).unwrap();
    } else {
        println!("Using existing database: {}", db_path);
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}", db_path))
        .await?;

    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    println!("Database setup complete");

    Ok(pool)
}