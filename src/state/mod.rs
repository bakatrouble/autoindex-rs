use std::sync::Arc;
use tokio::sync::RwLock;

mod config;
mod thumbnails;
mod events;

pub use config::Config;
pub use thumbnails::Thumbnails;
pub use events::Events;

pub struct State {
    pub config: Config,
    pub thumbnails: Thumbnails,
    pub events: Events,
}

pub type SharedState = Arc<RwLock<State>>;
