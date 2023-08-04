mod config;
mod err;
pub mod handler;
mod state;

pub use crate::config::*;
pub use err::{Error, Kind as ErrorKind};
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
