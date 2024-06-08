use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = config::parse();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    sqlx::migrate!().run(&db).await?;

    http::serve(config, db).await?;
}
