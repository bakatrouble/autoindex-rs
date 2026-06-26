mod endpoints;
mod state;

use std::sync::Arc;
use axum::{
    routing::{
        get,
        post
    },
    Router,
};
use axum::http::Request;
use tokio::{
    sync::RwLock,
    signal
};
use tower_http::{compression::CompressionLayer, trace, trace::TraceLayer};
use tracing::{Level, log::{warn}, Span};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use state::{State, SharedState, Config, Thumbnails, Events};

async fn shutdown_signal(state: SharedState) {
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

    warn!("Shutting down server");
    state.read().await.events.kill_all_connections().await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!("{}=debug,tower_http=info,axum=trace", env!("CARGO_CRATE_NAME")).into()
                })
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();
    let thumbnails = Thumbnails::new();
    let events = Events::new();
    let state = State {
        config: config.clone(),
        thumbnails,
        events,
    };
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    let app = Router::new()
        .route("/api/index", get(endpoints::index))
        .route("/api/thumbnail", get(endpoints::thumbnail))
        .route("/api/config", get(endpoints::config))
        .route("/api/events", get(endpoints::events))
        .route("/api/chroot", post(endpoints::chroot))
        .fallback(get(endpoints::main_handler))
        .with_state(Arc::clone(&shared_state))
        .layer((
                   CompressionLayer::new(),
                   TraceLayer::new_for_http()
                       .make_span_with(|request: &Request<_>| {
                           tracing::span!(
                               Level::INFO,
                               "request",
                               method = %request.method(),
                               uri = %request.uri(),
                               host = ?request.uri().host(),
                               version = ?request.version(),
                           )
                       })
                       .on_response(trace::DefaultOnResponse::new()
                           .level(Level::INFO)),
        ));

    let listener = tokio::net::TcpListener::bind(config.bind.as_str()).await.unwrap();
    println!("Listening on http://{}", config.bind.as_str());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(Arc::clone(&shared_state)))
        .await
        .unwrap()
}
