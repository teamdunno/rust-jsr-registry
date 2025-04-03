/// Metadata for JSR package
pub mod meta;
/// JSR package objects
pub mod package;
/// Module graphs from JSR package metadata
pub mod graph;
/// `scope` and `name` derivative
pub mod info;
/// Errors that are used in this package. Mostly are using [anyhow::Error]
pub mod error;