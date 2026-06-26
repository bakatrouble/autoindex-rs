mod endpoints;
mod state;

use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use axum::routing::post;
use tokio::sync::RwLock;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use state::{State, SharedState, Config, Thumbnails, Events};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let thumbnails = Thumbnails::new();
    let events = Events::new();
    let state = State {
        config: config.clone(),
        thumbnails,
        events,
    };
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    let api = Router::new()
        .route("/index", get(endpoints::index))
        .route("/thumbnail", get(endpoints::thumbnail))
        .route("/config", get(endpoints::config))
        .route("/events", get(endpoints::events))
        .route("/chroot", post(endpoints::chroot));

    let app = Router::new()
        .nest("/api", api)
        .fallback(get(endpoints::main_handler))
        .with_state(Arc::clone(&shared_state))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(config.bind.as_str()).await.unwrap();
    println!("Listening on http://{}", config.bind.as_str());
    axum::serve(listener, app).await.unwrap()
}
