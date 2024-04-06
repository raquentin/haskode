use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use haskode_primary_http::config::Config;
use haskode_primary_http::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::parse();

    env_logger::init();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    log::info!("successfully connected to database");

    //sqlx::migrate!().run(&db).await?;

    api::serve(config, db).await?;

    Ok(())
}
