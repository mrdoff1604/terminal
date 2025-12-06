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

    // 根据平台选择不同的PTY实现
    #[cfg(unix)]
    { 
        let factory = PortablePtyFactory::default();
        let pty = factory.create(&config).await?;
        Ok(pty)
    }
    #[cfg(not(unix))]
    { 
        let factory = MemoryPtyFactory::default();
        let pty = factory.create(&config).await?;
        Ok(pty)
    }
}

/// Create a new PTY instance using configuration from the application config
pub async fn create_pty_from_config(app_config: &crate::config::TerminalConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
    // Get default shell configuration
    let default_shell_type = &app_config.default_shell_type;
    let shell_config = match app_config.shells.get(default_shell_type) {
        Some(config) => config,
        None => {
            // If default shell is not found, use bash
            app_config.shells.get("bash").unwrap_or_else(|| {
                panic!("No shell configuration found for default shell: {}", default_shell_type)
            })
        }
    };
    
    // Extract command and arguments
    let mut command = shell_config.command.join(" ");
    let args: Vec<String> = shell_config.command.iter().skip(1).cloned().collect();
    
    // Create PTY config
    let pty_config = PtyConfig {
        command: command,
        args: args,
        cols: app_config.default_size.columns,
        rows: app_config.default_size.rows,
        env: shell_config.environment.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        cwd: shell_config.working_directory.clone(),
    };
    
    // 根据平台选择不同的PTY实现
    #[cfg(unix)]
    { 
        let factory = PortablePtyFactory::default();
        let pty = factory.create(&pty_config).await?;
        Ok(pty)
    }
    #[cfg(not(unix))]
    { 
        let factory = MemoryPtyFactory::default();
        let pty = factory.create(&pty_config).await?;
        Ok(pty)
    }
}

/// Create a new PTY instance with custom configuration
pub async fn create_pty_with_config(config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
    #[cfg(unix)]
    return PortablePtyFactory::default().create(config).await;
    
    #[cfg(not(unix))]
    return MemoryPtyFactory::default().create(config).await;
}

/// Create a new PTY instance using a specific factory
pub async fn create_pty_with_factory(
    factory: &dyn PtyFactory,
    config: &PtyConfig
) -> Result<Box<dyn AsyncPty>, PtyError> {
    factory.create(config).await
}
