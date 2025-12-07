/// REST API handlers for terminal session management

use axum::{extract::{State, Path, Json}, http::{StatusCode, Response}, routing::{get, post, delete}};
use axum::response::IntoResponse;
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;

use crate::{app_state::{AppState, Session, ConnectionType}, api::dto::{CreateSessionRequest, ResizeTerminalRequest, TerminalSession, ErrorResponse, SuccessResponse}};

/// Create a new terminal session
pub async fn create_session(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionRequest>
) -> impl IntoResponse {
    info!("Creating new terminal session for user: {}", req.user_id);
    
    // Generate a new session ID
    let session_id = Uuid::new_v4().to_string();
    
    // Create session with provided parameters
    let session = Session::new(
        session_id.clone(),
        req.user_id,
        req.title,
        req.working_directory,
        req.shell_type.unwrap_or_else(|| state.config.default_shell_type.clone()),
        req.columns.unwrap_or(80),
        req.rows.unwrap_or(24),
        ConnectionType::WebSocket,
    );
    
    // Add session to application state
    state.add_session(session.clone()).await;
    
    // Map to API response DTO
    let response = TerminalSession {
        session_id: session.session_id,
        user_id: session.user_id,
        title: session.title,
        status: format!("{:?}", session.status).to_lowercase(),
        columns: session.columns,
        rows: session.rows,
        working_directory: session.working_directory,
        shell_type: session.shell_type,
        connection_type: format!("{:?}", session.connection_type),
        created_at: session.created_at,
    };
    
    info!("Created session: {}", session_id);
    
    (StatusCode::CREATED, Json(response))
}

/// Get all terminal sessions
pub async fn get_all_sessions(
    State(state): State<AppState>
) -> impl IntoResponse {
    info!("Getting all terminal sessions");
    
    // Get all sessions from app state
    let sessions = state.get_all_sessions().await;
    
    // Map to API response DTOs
    let response_sessions: Vec<TerminalSession> = sessions.into_iter().map(|session| {
        TerminalSession {
            session_id: session.session_id,
            user_id: session.user_id,
            title: session.title,
            status: format!("{:?}", session.status).to_lowercase(),
            columns: session.columns,
            rows: session.rows,
            working_directory: session.working_directory,
            shell_type: session.shell_type,
            connection_type: format!("{:?}", session.connection_type),
            created_at: session.created_at,
        }
    }).collect();
    
    (StatusCode::OK, Json(response_sessions))
}

/// Get a specific terminal session by ID
pub async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>
) -> impl IntoResponse {
    info!("Getting terminal session: {}", session_id);
    
    // Get session from app state
    match state.get_session(&session_id).await {
        Some(session) => {
            // Return success as JSON value
            let success_response = json!(
                {
                    "session_id": session.session_id,
                    "user_id": session.user_id,
                    "title": session.title,
                    "status": format!("{:?}", session.status).to_lowercase(),
                    "columns": session.columns,
                    "rows": session.rows,
                    "working_directory": session.working_directory,
                    "shell_type": session.shell_type,
                    "connection_type": format!("{:?}", session.connection_type),
                    "created_at": session.created_at,
                }
            );
            
            (StatusCode::OK, Json(success_response))
        },
        None => {
            // Return error as JSON value
            let error_response = json!(
                {
                    "error": true,
                    "message": format!("Session not found: {}", session_id),
                    "code": 404
                }
            );
            
            (StatusCode::NOT_FOUND, Json(error_response))
        }
    }
}

/// Resize a terminal session
pub async fn resize_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(req): Json<ResizeTerminalRequest>
) -> impl IntoResponse {
    info!("Resizing terminal session: {} to {}x{}", session_id, req.columns, req.rows);
    
    // Get session from app state
    match state.get_session(&session_id).await {
        Some(mut session) => {
            // Update session size
            session.resize(req.columns, req.rows);
            
            // Update session in app state
            if state.update_session(session.clone()).await {
                // Return success response
                let success_response = json!(
                    {
                        "session_id": session_id,
                        "columns": req.columns,
                        "rows": req.rows,
                        "success": true
                    }
                );
                
                (StatusCode::OK, Json(success_response))
            } else {
                // Return error if update failed
                let error_response = json!(
                    {
                        "error": true,
                        "message": format!("Failed to update session: {}", session_id),
                        "code": 500
                    }
                );
                
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            }
        },
        None => {
            // Return error if session not found
            let error_response = json!(
                {
                    "error": true,
                    "message": format!("Session not found: {}", session_id),
                    "code": 404
                }
            );
            
            (StatusCode::NOT_FOUND, Json(error_response))
        }
    }
}

/// Terminate a terminal session
pub async fn terminate_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>
) -> impl IntoResponse {
    info!("Terminating terminal session: {}", session_id);
    
    // Remove session from app state
    match state.remove_session(&session_id).await {
        Some(session) => {
            // Return success response
            let success_response = json!(
                {
                    "session_id": session_id,
                    "success": true,
                    "reason": "Session terminated by API request"
                }
            );
            
            (StatusCode::OK, Json(success_response))
        },
        None => {
            // Return error if session not found
            let error_response = json!(
                {
                    "error": true,
                    "message": format!("Session not found: {}", session_id),
                    "code": 404
                }
            );
            
            (StatusCode::NOT_FOUND, Json(error_response))
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(SuccessResponse {
        success: true,
        message: "Health check passed".to_string()
    }))
}
