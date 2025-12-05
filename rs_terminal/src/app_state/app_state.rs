/// Application state implementation for Waylon Terminal Rust backend
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state containing shared data across handlers
#[derive(Clone)]
pub struct AppState {
    /// List of active session IDs
    pub sessions: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    /// Create a new instance of AppState
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
