mod portable_pty_impl;
/// PTY (Pseudo Terminal) handling for Waylon Terminal
/// This module wraps the blocking portable-pty operations into async functions
/// and provides a trait abstraction for different PTY implementations
mod pty_trait;

pub use portable_pty_impl::PortablePty;
pub use pty_trait::Pty;

/// Create a new PTY instance
/// This factory function returns the default PTY implementation (PortablePty)
pub async fn create_pty() -> Result<impl Pty, Box<dyn std::error::Error + Send>> {
    PortablePty::new().await
}
