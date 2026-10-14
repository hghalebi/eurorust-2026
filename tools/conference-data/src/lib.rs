//! Official facts, checked snapshots, portable exports, and itinerary checks.
pub mod collect;
pub mod domain;
pub mod export;
pub mod itinerary;
pub mod navigation;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid {kind}: {reason}")]
    Invalid { kind: &'static str, reason: String },
    #[error("source layout changed: {0}")]
    Layout(String),
    #[error("dataset validation failed: {0}")]
    Validation(String),
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON failed: {0}")]
    Json(#[from] serde_json::Error),
}

pub(crate) fn invalid(kind: &'static str, reason: impl Into<String>) -> Error {
    Error::Invalid {
        kind,
        reason: reason.into(),
    }
}
