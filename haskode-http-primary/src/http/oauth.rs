use crate::http::{ApiContext, Result};
use anyhow::Context;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::http::user;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/auth/github", get(auth_github))
        .route("/auth/github-callback", get(auth_callback))
}

#[derive(Deserialize)]
struct AuthQuery {
    code: String,
    state: String,
}

#[derive(Clone)]
pub struct OAuthState {
    pub client: Arc<BasicClient>,
    pub auth_session: Arc<Mutex<Option<AuthSession>>>,
}

pub struct AuthSession {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
}

async fn auth_github(ctx: State<ApiContext>) -> Result<Redirect> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = ctx
        .github
        .client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut auth_session = ctx.github.auth_session.lock().await;
    *auth_session = Some(AuthSession {
        pkce_verifier,
        csrf_token,
    });

    Ok(Redirect::temporary(auth_url.as_str()))
}

async fn auth_callback(
    ctx: State<ApiContext>,
    Query(query): Query<AuthQuery>,
) -> Result<&'static str> {
    let auth_session = {
        let mut auth_session_lock = ctx.github.auth_session.lock().await;
        auth_session_lock
            .take()
            .ok_or_else(|| anyhow::anyhow!("missing auth session"))?
    };

    if *auth_session.csrf_token.secret() != query.state {
        return Err(anyhow::anyhow!("CSRF token mismatch").into());
    }

    let token_result = ctx
        .github
        .client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(auth_session.pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("Failed to exchange code for token")?;

    let access_token = token_result.access_token().secret().to_string();

    let user_info = user::get_github_user_info(&access_token).await?;

    let existing_user = user::get_user_by_github_id(&ctx, &user_info.github_id).await;

    match existing_user {
        Ok(user) => {
            // Update the user with new access token and info if necessary
            user::update_user(
                &ctx,
                user::UpdateUser {
                    id: user.id,
                    github_id: user.github_id,
                    username: Some(user.username),
                    access_token: Some(access_token),
                },
            )
            .await?
        }
        Err(_) => {
            // Create a new user
            user::create_user(
                ctx,
                user::NewUser {
                    github_id: user_info.github_id,
                    username: user_info.username,
                    access_token,
                },
            )
            .await?
        }
    };

    Ok("GitHub Callback")
}
