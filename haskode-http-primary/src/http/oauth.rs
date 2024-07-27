use crate::http::{ApiContext, Result};
use anyhow::Context;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/auth/github", get(auth_github))
        .route("/auth/callback", get(auth_callback))
}

async fn auth_github() -> &'static str {
    "GitHub Auth"
}

async fn auth_callback() -> &'static str {
    "GitHub Callback"
}
