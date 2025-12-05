/// Protocol abstraction for Waylon Terminal Rust backend
mod connection;
mod websocket_connection;

pub use connection::{TerminalConnection, TerminalMessage, ConnectionType};
pub use websocket_connection::WebSocketConnection;
