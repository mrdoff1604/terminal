/// PTY (Pseudo Terminal) handling for Waylon Terminal
/// This module provides a trait abstraction for different PTY implementations
/// with a focus on pure async operations

mod pty_trait;
#[cfg(unix)]
mod portable_pty_impl;
mod memory_pty;

// Export all public types and traits
pub use pty_trait::*;
#[cfg(unix)]
pub use portable_pty_impl::{PortablePty, PortablePtyFactory};
pub use memory_pty::{MemoryPty, MemoryPtyFactory};

/// Create a new PTY instance with default configuration
/// This function returns a pure async PTY implementation
pub async fn create_pty() -> Result<Box<dyn AsyncPty>, PtyError> {
    // 创建配置
    let config = PtyConfig {
        command: "bash".to_string(),
        args: vec![],
        cols: 80,
        rows: 24,
        env: vec![
            ("TERM".to_string(), "xterm-256color".to_string()),
            ("COLORTERM".to_string(), "truecolor".to_string()),
        ],
        cwd: None,
    };

    // 创建PTY - 使用MemoryPtyFactory作为默认实现
    // PortablePtyFactory可以用于生产环境
    let factory = MemoryPtyFactory::default();
    let pty = factory.create(&config).await?;
    Ok(pty)
}

/// Create a new PTY instance with custom configuration
pub async fn create_pty_with_config(config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
    MemoryPtyFactory::default().create(config).await
}

/// Create a new PTY instance using a specific factory
pub async fn create_pty_with_factory(
    factory: &dyn PtyFactory,
    config: &PtyConfig
) -> Result<Box<dyn AsyncPty>, PtyError> {
    factory.create(config).await
}
