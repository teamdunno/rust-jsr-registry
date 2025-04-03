use thiserror::Error;


#[derive(Error, Debug)]
pub enum NpmCompParseError {
    #[error("Input does not start with @{}/", .0)]
    DosentStartWithPrefix(String),
    #[error("Input does not have the correct format (scope__name)")]
    CompFormat,
}