/// Errors that can arise within the TEZZERA framework.
#[derive(Debug)]
pub enum TezzeraError {
    /// A required resource could not be located.
    NotFound { resource: &'static str },
    /// The system is in an unexpected or inconsistent state.
    InvalidState(String),
    /// A layout computation failed or produced an invalid result.
    LayoutError(String),
    /// An unexpected internal error occurred.
    Internal(String),
}

impl TezzeraError {
    /// Creates a `NotFound` error for the given resource name.
    pub fn not_found(resource: &'static str) -> Self {
        TezzeraError::NotFound { resource }
    }

    /// Creates an `Internal` error with an arbitrary message.
    pub fn internal(msg: impl Into<String>) -> Self {
        TezzeraError::Internal(msg.into())
    }
}

impl std::fmt::Display for TezzeraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TezzeraError::NotFound { resource } => write!(f, "resource not found: {resource}"),
            TezzeraError::InvalidState(msg) => write!(f, "invalid state: {msg}"),
            TezzeraError::LayoutError(msg) => write!(f, "layout error: {msg}"),
            TezzeraError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for TezzeraError {}

/// Convenience alias for `Result<T, TezzeraError>`.
pub type TezzeraResult<T> = Result<T, TezzeraError>;
