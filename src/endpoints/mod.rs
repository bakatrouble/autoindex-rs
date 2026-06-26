mod index;
mod thumbnail;
mod main;
mod config;
mod events;
mod chroot;

use serde::Serialize;
pub use index::*;
pub use thumbnail::*;
pub use main::*;
pub use config::*;
pub use events::*;
pub use chroot::*;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}
