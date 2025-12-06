/// PTY (Pseudo Terminal) handling for Waylon Terminal
/// This module wraps the blocking portable-pty operations into async functions
/// and provides a trait abstraction for different PTY implementations

mod pty_trait;
mod portable_pty_impl;
mod mock_pty;

pub use pty_trait::Pty;
pub use portable_pty_impl::PortablePty;
pub use mock_pty::MockPty;

/// Create a new PTY instance
/// This factory function returns the mock PTY implementation for testing purposes
pub async fn create_pty() -> Result<impl Pty, Box<dyn std::error::Error + Send>> {
    MockPty::new().await
}
