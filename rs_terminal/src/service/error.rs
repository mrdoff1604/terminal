/// Error types for the service layer
use std::fmt;

/// Service layer error type
#[derive(Debug)]
pub enum ServiceError {
    /// IO error
    Io(std::io::Error),
    /// PTY error
    Pty(crate::pty::PtyError),
    /// Connection error
    Connection(Box<dyn std::error::Error + Send>),
    /// Other error
    Other(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::Io(e) => write!(f, "IO error: {}", e),
            ServiceError::Pty(e) => write!(f, "PTY error: {}", e),
            ServiceError::Connection(e) => write!(f, "Connection error: {}", e),
            ServiceError::Other(s) => write!(f, "Other error: {}", s),
        }
    }
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::Io(e) => Some(e),
            ServiceError::Pty(e) => Some(e),
            ServiceError::Connection(e) => Some(e.as_ref()),
            ServiceError::Other(_) => None,
        }
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(e: std::io::Error) -> Self {
        ServiceError::Io(e)
    }
}

impl From<crate::pty::PtyError> for ServiceError {
    fn from(e: crate::pty::PtyError) -> Self {
        ServiceError::Pty(e)
    }
}

impl From<Box<dyn std::error::Error + Send>> for ServiceError {
    fn from(e: Box<dyn std::error::Error + Send>) -> Self {
        ServiceError::Connection(e)
    }
}
