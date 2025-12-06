/// Configuration file loader for rs_terminal
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use tracing::{info, error};
use crate::config::TerminalConfig;

/// Configuration loader responsible for loading and parsing configuration files
pub struct ConfigLoader;

impl ConfigLoader {
    /// Create a new configuration loader
    pub fn new() -> Self {
        Self
    }

    /// Load configuration from a file, or return default if file doesn't exist
    pub fn load_config(&self, config_path: Option<&Path>) -> TerminalConfig {
        match config_path {
            Some(path) => {
                self.load_config_from_file(path)
            },
            None => {
                info!("No configuration file specified, using default configuration");
                TerminalConfig::default()
            }
        }
    }

    /// Load configuration from a specific file path
    fn load_config_from_file(&self, path: &Path) -> TerminalConfig {
        info!("Loading configuration from file: {:?}", path);
        
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    error!("Failed to read configuration file: {}, using default configuration", e);
                    return TerminalConfig::default();
                }
                
                self.parse_config(&contents)
            },
            Err(e) => {
                error!("Failed to open configuration file: {}, using default configuration", e);
                TerminalConfig::default()
            }
        }
    }

    /// Parse configuration from string content
    fn parse_config(&self, content: &str) -> TerminalConfig {
        match toml::from_str::<TerminalConfig>(content) {
            Ok(config) => {
                info!("Configuration parsed successfully");
                config
            },
            Err(e) => {
                error!("Failed to parse configuration: {}, using default configuration", e);
                TerminalConfig::default()
            }
        }
    }
}

/// Default configuration path
pub fn default_config_path() -> Option<std::path::PathBuf> {
    // Try to get the executable directory
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|dir| dir.join("config.toml")))
}
