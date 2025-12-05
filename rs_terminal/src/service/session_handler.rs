/// Terminal session handler for processing terminal connections
use tokio::select;
use tracing::{info, error, debug};

use crate::{app_state::AppState, protocol::TerminalConnection};
use crate::pty::Pty;

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
    
    // Create PTY for this session using factory function
    let mut pty = match crate::pty::create_pty().await {
        Ok(pty) => pty,
        Err(e) => {
            error!("Failed to create PTY for session {}: {}", conn_id, e);
            // Send error message and close connection
            let _ = connection.send_text(&format!("Error: Failed to create terminal session: {}", e)).await;
            let _ = connection.close().await;
            // Clean up session
            let mut sessions = state.sessions.lock().await;
            sessions.retain(|id| id != &conn_id);
            drop(sessions);
            return;
        }
    };
    
    info!("PTY created for session {}", conn_id);
    
    // Create a buffer for reading from PTY
    let mut pty_buffer = [0u8; 4096];
    
    // Main session loop - handle both incoming messages and PTY output
    loop {
        select! {
            // Handle incoming messages from the connection
            msg_result = connection.receive() => {
                match msg_result {
                    Some(Ok(msg)) => {
                        match msg {
                            crate::protocol::TerminalMessage::Text(text) => {
                                debug!("Received text message from session {}: {}", conn_id, text);
                                // Write the text to PTY
                                if let Err(e) = pty.write(text.as_bytes()).await {
                                    error!("Failed to write to PTY for session {}: {}", conn_id, e);
                                    break;
                                }
                            }
                            crate::protocol::TerminalMessage::Binary(bin) => {
                                debug!("Received binary message from session {} of length {}", conn_id, bin.len());
                                // Write binary data to PTY
                                if let Err(e) = pty.write(&bin).await {
                                    error!("Failed to write binary data to PTY for session {}: {}", conn_id, e);
                                    break;
                                }
                            }
                            crate::protocol::TerminalMessage::Ping(_) => {
                                debug!("Received ping from session {}", conn_id);
                                // Respond with pong
                                if let Err(e) = connection.send_text(&"Pong").await {
                                    error!("Failed to send pong response to session {}: {}", conn_id, e);
                                    break;
                                }
                            }
                            crate::protocol::TerminalMessage::Pong(_) => {
                                debug!("Received pong from session {}", conn_id);
                                // Pong received, do nothing
                            }
                            crate::protocol::TerminalMessage::Close => {
                                info!("Received close message from session {}", conn_id);
                                break;
                            }
                        }
                    }
                    Some(Err(e)) => {
                        error!("Connection error for session {}: {}", conn_id, e);
                        break;
                    }
                    None => {
                        // Connection closed
                        info!("Connection closed by client for session {}", conn_id);
                        break;
                    }
                }
            },
            // Handle PTY output
            read_result = pty.read(&mut pty_buffer) => {
                match read_result {
                    Ok(read_bytes) => {
                        if read_bytes > 0 {
                            debug!("Read {} bytes from PTY for session {}", read_bytes, conn_id);
                            // Send the PTY output back to the connection
                            let output = &pty_buffer[..read_bytes];
                            if let Err(e) = connection.send_binary(output).await {
                                error!("Failed to send PTY output to session {}: {}", conn_id, e);
                                break;
                            }
                        }
                        // Don't break when read_bytes is 0 - it might just be a timeout
                    }
                    Err(e) => {
                        error!("Error reading from PTY for session {}: {}", conn_id, e);
                        break;
                    }
                }
            },
        }
    }
    
    // Clean up resources
    info!("Cleaning up session {}", conn_id);
    
    // Close the connection
    if let Err(e) = connection.close().await {
        error!("Failed to close connection for session {}: {}", conn_id, e);
    }
    
    // Kill the PTY process
    if let Err(e) = pty.kill().await {
        error!("Failed to kill PTY process for session {}: {}", conn_id, e);
    }
    
    // Remove session from state
    let mut sessions = state.sessions.lock().await;
    sessions.retain(|id| id != &conn_id);
    drop(sessions);
    
    info!("Terminal session {} closed", conn_id);
}
