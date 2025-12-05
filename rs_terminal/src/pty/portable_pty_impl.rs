use std::sync::{Arc, Mutex};
use tracing::{info, error};
use futures_util::future::FutureExt;

use crate::pty::Pty;

/// Portable PTY implementation using the portable-pty crate
/// This is a simplified implementation that uses empty methods for now
pub struct PortablePty {
    /// Flag to track if the PTY is alive
    alive: Arc<Mutex<bool>>,
}

impl PortablePty {
    /// Create a new PortablePty instance
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send>> {
        info!("Creating new PortablePty instance");
        Ok(Self {
            alive: Arc::new(Mutex::new(true)),
        })
    }
}

#[async_trait::async_trait]
impl Pty for PortablePty {
    async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send>> {
        info!("Write called with {} bytes", data.len());
        Ok(())
    }
    
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error + Send>> {
        info!("Read called with buffer size {}", buffer.len());
        // Return 0 bytes read for now
        Ok(0)
    }
    
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), Box<dyn std::error::Error + Send>> {
        info!("Resize request received, but not implemented yet. cols: {}, rows: {}", cols, rows);
        Ok(())
    }
    
    async fn kill(&mut self) -> Result<(), Box<dyn std::error::Error + Send>> {
        info!("Kill called");
        *self.alive.lock().unwrap() = false;
        Ok(())
    }
    
    async fn is_alive(&self) -> Result<bool, Box<dyn std::error::Error + Send>> {
        Ok(*self.alive.lock().unwrap())
    }
}

impl Drop for PortablePty {
    /// Clean up resources when the PortablePty is dropped
    fn drop(&mut self) {
        info!("Dropping PortablePty, cleaning up resources");
        
        // Try to kill the process if it's still running
        if let Some(Err(e)) = self.kill().now_or_never() {
            error!("Failed to kill PTY process on drop: {:?}", e);
        }
    }
}