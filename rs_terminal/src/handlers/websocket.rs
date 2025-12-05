use axum::{extract::State, response::IntoResponse, extract::ws::{WebSocket, WebSocketUpgrade}};
use axum::extract::ws::Message::{Text, Binary, Ping, Pong, Close};
use futures_util::StreamExt;
use tracing::{info, error};

use crate::app_state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let state_clone = state.clone();
    ws.on_upgrade(|socket| handle_socket(socket, state_clone))
}

pub async fn handle_socket(
    mut socket: WebSocket,
    state: AppState,
) {
    info!("New WebSocket connection");
    
    // Add session to state
    let mut sessions = state.sessions.lock().await;
    let session_id = format!("session-{}", sessions.len());
    sessions.push(session_id.clone());
    drop(sessions);
    
    // Send welcome message
    let welcome_msg = Text(format!("Welcome to Waylon Terminal! Your session ID: {}", session_id));
    if socket.send(welcome_msg).await.is_err() {
        info!("Failed to send welcome message");
        return;
    }
    
    // Handle messages
    while let Some(msg) = socket.next().await {
        match msg {
            Ok(msg) => {
                match msg {
                    Text(text) => {
                        info!("Received message: {}", text);
                        // Echo back the message
                        if socket.send(Text(format!("Echo: {}", text))).await.is_err() {
                            break;
                        }
                    }
                    Binary(_) => {
                        info!("Received binary message");
                    }
                    Ping(_) => {
                        if socket.send(Pong(Vec::new())).await.is_err() {
                            break;
                        }
                    }
                    Pong(_) => {
                        // Do nothing for pong messages
                    }
                    Close(_) => {
                        info!("WebSocket connection closed by client");
                        break;
                    }
                    _ => {
                        info!("Received unhandled message type");
                    }
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    // Remove session from state
    let mut sessions = state.sessions.lock().await;
    sessions.retain(|id| id != &session_id);
    
    info!("WebSocket connection closed");
}
