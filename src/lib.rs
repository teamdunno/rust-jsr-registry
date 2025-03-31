#![forbid(unsafe_code)]
#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
/// Package fetcher objects
pub mod fetcher;
mod types;
pub use types::*;
mod default_url;
pub use default_url::DEFAULT_URL;
