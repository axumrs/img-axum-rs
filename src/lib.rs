mod config;
mod err;
pub mod handler;
pub mod meta;
mod state;
pub mod view;

pub use crate::config::*;
pub use err::{Error, Kind as ErrorKind};
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;
