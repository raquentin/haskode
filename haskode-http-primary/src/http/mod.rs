use crate::config::Config;
use anyhow::Context;
use oauth::OAuthState;
use oauth2::basic::BasicClient;
use axum::{http::header::AUTHORIZATION, Router};
use axum::routing::get;
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod error;

mod oauth;
mod user;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub github: OAuthState,
}

pub async fn serve(config: Config, db: PgPool, github: BasicClient) -> anyhow::Result<()> {

    let github = OAuthState {
        client: Arc::new(github),
        auth_session: Arc::new(Mutex::new(None)),
    };

    let api_context = ApiContext {
        config: Arc::new(config),
        db,
        github,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .route("/", get(|| async { "hello" }))
        .merge(oauth::router())
        .layer((
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
            CompressionLayer::new(),
            TraceLayer::new_for_http().on_failure(()),
            TimeoutLayer::new(Duration::from_secs(30)),
            CatchPanicLayer::new(),
        ))
        .with_state(api_context)
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
