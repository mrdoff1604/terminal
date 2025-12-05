/// PTY (Pseudo Terminal) handling for Waylon Terminal
/// This module wraps the blocking portable-pty operations into async functions
/// and provides a trait abstraction for different PTY implementations

mod pty_trait;
mod portable_pty_impl;

pub use pty_trait::Pty;
pub use portable_pty_impl::PortablePty;