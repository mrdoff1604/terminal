/// Main entry point for Waylon Terminal Rust backend

// Import modules
mod app_state;
mod config;
mod handlers;
mod server;

// Use public API from modules
use app_state::AppState;
use config::init_logging;
use server::{build_router, run_server, start_webtransport_service};

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();
    
    // Create application state
    let app_state = AppState::new();
    
    // Start WebTransport service
    start_webtransport_service(app_state.clone());
    
    // Build router and run server
    let app = build_router(app_state);
    run_server(app).await;
}



