use anyhow::Context;
use anyhow::anyhow;
use axum::extract::State;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::{ApiContext, Result};

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub github_id: String,
    pub username: String,
    pub access_token: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub struct NewUser {
    pub github_id: String,
    pub username: String,
    pub access_token: String,
} 

pub async fn create_user(
    ctx: State<ApiContext>,
    user: NewUser,
) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (github_id, username, access_token, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, github_id, username, access_token, created_at
        "#,
        user.github_id,
        user.username,
        user.access_token,
        chrono::Utc::now().naive_utc(),
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(user)
}

pub struct UpdateUser {
    pub id: Uuid,
    pub github_id: String,
    pub username: Option<String>,
    pub access_token: Option<String>,
}

pub async fn update_user(
    ctx: &State<ApiContext>,
    user: UpdateUser
) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            username = COALESCE($1, username),
            access_token = COALESCE($2, access_token)
        WHERE id = $3
        RETURNING id, github_id, username, access_token, created_at
        "#,
        user.username,
        user.access_token,
        user.id,
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(user)
}

#[derive(Deserialize)]
pub struct GitHubUserInfo {
    pub github_id: String,
    #[serde(rename = "login")]
    pub username: String,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
}

pub async fn get_github_user_info(access_token: &str) -> Result<GitHubUserInfo> {
    let client = Client::new();
    let user_info_url = "https://api.github.com/user";

    let response = client
        .get(user_info_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "Haskode")
        .send()
        .await
        .context("failed to fetch user info from GitHub")?;

    if !response.status().is_success() {
        return Err(Error::Anyhow(anyhow!("failed to fetch user info from GitHub")));
    }

    let user_info: GitHubUserInfo = response.json().await.context("failed to parse user info")?;

    Ok(user_info)
}

pub async fn get_user_by_github_id(ctx: &State<ApiContext>, github_id: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, github_id, username, access_token, created_at
        FROM users
        WHERE github_id = $1
        "#,
        github_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(user)
}
