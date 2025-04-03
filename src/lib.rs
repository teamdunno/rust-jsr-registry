#![forbid(unsafe_code)]
#![forbid(clippy::implicit_return)]
#![forbid(unused_imports)]
#![allow(clippy::needless_return)]
/// Package fetcher objects
pub mod fetcher;
mod types;
pub use types::*;
mod default_url;
pub use default_url::DEFAULT_URL;
pub use default_url::DEFAULT_NPM_COMP_URL;
mod private_tools;
mod serde;