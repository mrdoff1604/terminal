/// Configuration data structures for rs_terminal
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Terminal configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerminalConfig {
    /// Default shell type
    pub default_shell_type: String,
    
    /// Default terminal dimensions
    pub default_size: TerminalSize,
    
    /// Default working directory
    pub default_working_directory: PathBuf,
    
    /// Session timeout in milliseconds (default: 30 minutes)
    pub session_timeout: u64,
    
    /// Shell configurations
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

/// Shell configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShellConfig {
    /// Command to execute
    pub command: Vec<String>,
    
    /// Working directory (optional)
    pub working_directory: Option<PathBuf>,
    
    /// Terminal size (optional)
    pub size: Option<TerminalSize>,
    
    /// Environment variables
    pub environment: std::collections::HashMap<String, String>,
}

/// Default configuration values
impl Default for TerminalConfig {
    fn default() -> Self {
        // 根据平台选择默认shell类型
        let default_shell_type = if cfg!(windows) {
            "cmd".to_string()
        } else {
            "bash".to_string()
        };

        let mut shells = std::collections::HashMap::new();
        
        // 添加bash配置（适用于Unix/Linux系统）
        shells.insert("bash".to_string(), ShellConfig {
            command: vec!["bash".to_string(), "-l".to_string()],
            working_directory: None,
            size: None,
            environment: {
                let mut env = std::collections::HashMap::new();
                env.insert("TERM".to_string(), "xterm-256color".to_string());
                env.insert("PATH".to_string(), "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_string());
                env
            },
        });

        // 添加cmd配置（适用于Windows系统）
        shells.insert("cmd".to_string(), ShellConfig {
            command: vec!["cmd.exe".to_string(), "/c".to_string(), "cmd".to_string()],
            working_directory: None,
            size: None,
            environment: {
                let mut env = std::collections::HashMap::new();
                env.insert("TERM".to_string(), "xterm-256color".to_string());
                env
            },
        });

        // 添加powershell配置（适用于Windows系统）
        shells.insert("powershell".to_string(), ShellConfig {
            command: vec!["powershell.exe".to_string()],
            working_directory: None,
            size: None,
            environment: {
                let mut env = std::collections::HashMap::new();
                env.insert("TERM".to_string(), "xterm-256color".to_string());
                env
            },
        });
        
        Self {
            default_shell_type,
            default_size: TerminalSize {
                columns: 80,
                rows: 24,
            },
            default_working_directory: PathBuf::from("."),
            session_timeout: 1800000, // 30 minutes
            shells,
        }
    }
}
