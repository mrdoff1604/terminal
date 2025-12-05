/// WebTransport connection implementation for TerminalConnection trait
use std::fmt::Debug;
use tracing::info;

use crate::protocol::{TerminalConnection, TerminalMessage, ConnectionType};

/// WebTransport connection implementation that implements TerminalConnection trait
/// This follows the same pattern as WebSocketConnection
pub struct WebTransportConnection {
    pub id: String,
    // We'll use a placeholder for the session for now
    // The actual session type will be added when we have the correct wtransport API
    _session: (),
}

impl Debug for WebTransportConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebTransportConnection")
            .field("id", &self.id)
            .finish()
    }
}

impl WebTransportConnection {
    /// Create a new WebTransport connection
    pub fn new(id: String) -> Self {
        Self {
            id,
            _session: (),
        }
    }
}

#[async_trait::async_trait]
impl TerminalConnection for WebTransportConnection {
    async fn send_text(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error + Send>> {
        // WebTransport implementation for sending text messages
        // This will be implemented later with actual WebTransport logic
        info!("WebTransport send_text called: {}", message);
        Ok(())
    }
    
    async fn send_binary(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send>> {
        // WebTransport implementation for sending binary messages
        // This will be implemented later with actual WebTransport logic
        info!("WebTransport send_binary called with {} bytes", data.len());
        Ok(())
    }
    
    async fn receive(&mut self) -> Option<Result<TerminalMessage, Box<dyn std::error::Error + Send>>> {
        // WebTransport implementation for receiving messages
        // This will be implemented later with actual WebTransport logic
        // For now, we'll just return None to indicate no messages available
        None
    }
    
    async fn close(&mut self) -> Result<(), Box<dyn std::error::Error + Send>> {
        // WebTransport implementation for closing the connection
        // This will be implemented later with actual WebTransport logic
        info!("WebTransport close called");
        Ok(())
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn connection_type(&self) -> ConnectionType {
        ConnectionType::WebTransport
    }
}
