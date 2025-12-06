/// Server implementation for Waylon Terminal Rust backend
use std::net::SocketAddr;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;

use crate::{app_state::AppState, handlers::websocket};

/// Start WebTransport server in a separate task
pub fn start_webtransport_service(state: AppState) {
    let webtransport_addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    let webtransport_state = state.clone();
    tokio::spawn(async move {
        crate::handlers::webtransport::start_webtransport_server(
            webtransport_addr,
            webtransport_state,
        )
        .await;
    });
}

/// Build the application router with routes
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Waylon Terminal - Rust Backend" }))
        .route("/ws", get(websocket::websocket_handler))
        .with_state(state)
}

/// Run the HTTP server
pub async fn run_server(router: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let webtransport_addr = SocketAddr::from(([0, 0, 0, 0], 8082));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running on http://{}", addr);
    info!("WebSocket server available at ws://{}/ws", addr);
    info!(
        "WebTransport server available at https://{}",
        webtransport_addr
    );

    axum::serve(listener, router).await.unwrap();
}
