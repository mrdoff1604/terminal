/// Configuration data structures for rs_terminal
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Terminal configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerminalConfig {
    /// Default shell type
    pub default_shell_type: String,
    
    /// Session timeout in milliseconds (default: 30 minutes)
    pub session_timeout: u64,
    
    /// Default shell configuration (used as fallback for all shells)
    pub default_shell_config: DefaultShellConfig,
    
    /// Shell configurations (specific shell types)
    pub shells: std::collections::HashMap<String, ShellConfig>,
}

/// Terminal size configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerminalSize {
    /// Number of columns
    pub columns: u16,
    
    /// Number of rows
    pub rows: u16,
}

/// Default shell configuration (used as fallback template)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultShellConfig {
    /// Terminal size (required in default config)
    pub size: TerminalSize,
    
    /// Working directory (optional)
    pub working_directory: Option<PathBuf>,
    
    /// Environment variables (optional)
    pub environment: Option<std::collections::HashMap<String, String>>,
}

/// Shell configuration for specific shell types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShellConfig {
    /// Command to execute (required for each shell type)
    pub command: Vec<String>,
    
    /// Working directory (optional, defaults to default_shell_config.working_directory)
    pub working_directory: Option<PathBuf>,
    
    /// Terminal size (optional, defaults to default_shell_config.size)
    pub size: Option<TerminalSize>,
    
    /// Environment variables (optional, defaults to default_shell_config.environment)
    pub environment: Option<std::collections::HashMap<String, String>>,
}

// 删除了硬编码的默认配置，所有配置必须从配置文件读取
