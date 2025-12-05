/// Terminal session handler for processing terminal connections
use tracing::{info, error};

use crate::{app_state::AppState, protocol::TerminalConnection};

/// Handle a terminal session using the TerminalConnection trait
pub async fn handle_terminal_session(
    mut connection: impl TerminalConnection,
    state: AppState
) {
    let conn_id = connection.id().to_string();
    let conn_type = connection.connection_type();
    
    info!("New terminal connection: {} (Type: {:?})", conn_id, conn_type);
    
    // Add session to state
    let mut sessions = state.sessions.lock().await;
    sessions.push(conn_id.clone());
    drop(sessions);
    
    // Send welcome message
    let welcome_msg = format!("Welcome to Waylon Terminal! Your session ID: {}", conn_id);
    if let Err(e) = connection.send_text(&welcome_msg).await {
        error!("Failed to send welcome message to session {}: {}", conn_id, e);
        return;
    }
    
    // Handle incoming messages
    while let Some(msg_result) = connection.receive().await {
        match msg_result {
            Ok(msg) => {
                match msg {
                    crate::protocol::TerminalMessage::Text(text) => {
                        info!("Received message: {} from session {}", text, conn_id);
                        
                        // Echo back the message
                        let response = format!("Echo: {}", text);
                        if let Err(e) = connection.send_text(&response).await {
                            error!("Failed to send response to session {}: {}", conn_id, e);
                            break;
                        }
                    }
                    crate::protocol::TerminalMessage::Binary(bin) => {
                        info!("Received binary message from session {} of length {}", conn_id, bin.len());
                        // Echo back binary messages
                        if let Err(e) = connection.send_binary(&bin).await {
                            error!("Failed to send binary response to session {}: {}", conn_id, e);
                            break;
                        }
                    }
                    crate::protocol::TerminalMessage::Ping(_ping) => {
                        info!("Received ping from session {}", conn_id);
                        // Echo back ping as pong
                        if let Err(e) = connection.send_text(&format!("Pong received")).await {
                            error!("Failed to send pong response to session {}: {}", conn_id, e);
                            break;
                        }
                    }
                    crate::protocol::TerminalMessage::Pong(_) => {
                        info!("Received pong from session {}", conn_id);
                    }
                    crate::protocol::TerminalMessage::Close => {
                        info!("Received close message from session {}", conn_id);
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Connection error for session {}: {}", conn_id, e);
                break;
            }
        }
    }
    
    // Close the connection
    if let Err(e) = connection.close().await {
        error!("Failed to close connection for session {}: {}", conn_id, e);
    }
    
    // Remove session from state
    let mut sessions = state.sessions.lock().await;
    sessions.retain(|id| id != &conn_id);
    drop(sessions);
    
    info!("Terminal session {} closed", conn_id);
}
