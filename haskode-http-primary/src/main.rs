use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use haskode_http_primary::config::Config;
use haskode_http_primary::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    //sqlx::migrate!("../../migrations").run(&db).await?;

    http::serve(config, db).await?;
    Ok(())
}
