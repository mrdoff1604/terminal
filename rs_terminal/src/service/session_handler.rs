/// Terminal session handler for processing terminal connections
use tokio::select;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info};

use crate::{app_state::AppState, protocol::TerminalConnection};

/// Handle a terminal session using the TerminalConnection trait
pub async fn handle_terminal_session(mut connection: impl TerminalConnection, state: AppState) {
    let conn_id = connection.id().to_string();
    let conn_type = connection.connection_type();

    info!(
        "New terminal connection: {} (Type: {:?})",
        conn_id, conn_type
    );

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
            let _ = connection
                .send_text(&format!("Error: Failed to create terminal session: {}", e))
                .await;
            let _ = connection.close().await;
            // Clean up session
            let mut sessions = state.sessions.lock().await;
            sessions.retain(|id| id != &conn_id);
            drop(sessions);
            return;
        }
    };

    info!("PTY created for session {}", conn_id);

    // Main session loop - handle both incoming messages and PTY output directly
    let mut pty_buffer = [0u8; 4096];
    loop {
        select! {
            // Handle incoming messages from the connection
            msg_result = connection.receive() => {
                match msg_result {
                    Some(Ok(msg)) => {
                        match msg {
                            crate::protocol::TerminalMessage::Text(text) => {
                                debug!("Received text message from session {}: {}", conn_id, text);
                                // Write the text to PTY directly (non-blocking async)
                                if let Err(e) = pty.write(text.as_bytes()).await {
                                    error!("Failed to write to PTY for session {}: {}", conn_id, e);
                                    break;
                                }
                            }
                            crate::protocol::TerminalMessage::Binary(bin) => {
                                debug!("Received binary message from session {} of length {}", conn_id, bin.len());
                                // Write binary data to PTY directly (non-blocking async)
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
            // Handle PTY output directly (non-blocking async)
            read_result = pty.read(&mut pty_buffer) => {
                match read_result {
                    Ok(0) => {
                        // EOF - PTY has closed
                        info!("PTY closed for session {}", conn_id);
                        break;
                    }
                    Ok(n) => {
                        // PTY output received
                        debug!("Received {} bytes from PTY for session {}", n, conn_id);
                        
                        let data = &pty_buffer[..n];
                        // Print the data in a human-readable format
                        debug!("PTY data: {:?}", String::from_utf8_lossy(data));
                        
                        // Try to convert data to string for text-based protocols
                        match String::from_utf8(data.to_vec()) {
                            Ok(text) => {
                                // Send text to client
                                if let Err(e) = connection.send_text(&text).await {
                                    error!("Failed to send PTY text output to session {}: {}", conn_id, e);
                                    break;
                                }
                            },
                            Err(_) => {
                                // Send as binary if conversion fails
                                if let Err(e) = connection.send_binary(data).await {
                                    error!("Failed to send PTY binary output to session {}: {}", conn_id, e);
                                    break;
                                }
                            }
                        }
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
