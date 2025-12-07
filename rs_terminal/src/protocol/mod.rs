/// Protocol abstraction for Waylon Terminal Rust backend
mod connection;
mod websocket_connection;

pub use connection::{ConnectionType, TerminalConnection, TerminalMessage};
pub use websocket_connection::WebSocketConnection;
