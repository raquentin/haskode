use anyhow::Context;
use clap::Parser;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sqlx::postgres::PgPoolOptions;

use haskode_http_primary::config::Config;
use haskode_http_primary::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = Config::parse();

    let connection_string = format!(
        "postgres://{}:{}@localhost:5432/{}",
        &config.postgres_user, &config.postgres_password, &config.postgres_user
    );

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&connection_string)
        .await
        .context("Failed to connect to database")?;

    //sqlx::migrate!("../../migrations").run(&db).await?;

    let github_client_id = ClientId::new(config.github_client_id.clone());
    let github_client_secret = ClientSecret::new(config.github_client_secret.clone());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url =
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL");
    let github = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);

    log::info!("server started");

    http::serve(config, db, github).await?;

    Ok(())
}
