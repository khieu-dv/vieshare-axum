use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod app;
mod controllers;
mod errors;
mod forms;
mod models;
mod routes;
mod settings;
mod utils;

// Database module
mod db {
    include!("../db/db.rs");
}

// Tests removed for now - can be added later as needed


use settings::SETTINGS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let pool = db::create_pool().await?;


    let app = app::create_app(pool).await;

    let listener = TcpListener::bind(address).await?;
    info!("Server listening on {}", &address);

    axum::serve(listener, app).await?;

    Ok(())
}