use std::sync::Arc;
use tokio::sync::RwLock;
pub use crate::state::config::Config;
pub use crate::state::thumbnails::Thumbnails;
pub use crate::state::events::Events;

pub mod config;
pub mod thumbnails;
mod events;

pub struct State {
    pub config: Config,
    pub thumbnails: Thumbnails,
    pub events: Events,
}

pub type SharedState = Arc<RwLock<State>>;