use anyhow::Result;
use axum::{response::IntoResponse, routing::post, Extension, Router};
use clap::Parser;
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;

// Server configuration
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct ServerConfig {
    #[clap(short = 's', long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "3000")]
    port: u16,

    #[clap(short, long, default_value = "content.txt")]
    file_path: String,
}

// App state containing the file content
#[derive(Clone)]
struct AppState {
    file_path: String,
}

// Request body structure for file path
#[derive(serde::Deserialize)]
struct FileRequest {
    file_path: Option<String>,
}

// Handler for the file content endpoint
async fn get_file_content(
    Extension(state): Extension<Arc<AppState>>,
    axum::Json(request): axum::Json<FileRequest>
) -> impl IntoResponse {
    // Use file path from request body if provided, otherwise use default
    let file_path = request.file_path.as_ref().unwrap_or(&state.file_path);
    
    let result = read_to_string(file_path);
    result.unwrap_or_else(|_| format!("Failed to read file: {}", file_path))
}

/// Create and configure the Axum router
fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/file", post(get_file_content))
        .layer(Extension(state))
}

/// Parse the socket address from configuration
fn parse_socket_addr(config: &ServerConfig) -> Result<SocketAddr> {
    let addr_str = format!("{}:{}", config.host, config.port);
    addr_str.parse().map_err(|e| anyhow::anyhow!("Invalid socket address: {}", e))
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
async fn wait_for_shutdown() -> Result<()> {
    // Wait for either Ctrl+C or SIGTERM signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, shutting down...");
        },
        _ = async {
            // Only listen for SIGTERM on Unix systems
            #[cfg(unix)]
            {
                if let Ok(mut sigterm) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
                    sigterm.recv().await;
                    println!("\nReceived SIGTERM, shutting down...");
                }
            }
            // For Windows, just wait indefinitely
            #[cfg(windows)]
            {
                std::future::pending::<()>().await;
            }
        } => {},
    };
    Ok(())
}

/// Start the server and handle graceful shutdown
async fn run_server(config: ServerConfig) -> Result<()> {
    let addr = parse_socket_addr(&config)?;
    
    println!("Server listening on http://{}", addr);
    println!("Serving file: {}", config.file_path);
    println!("Press Ctrl+C to gracefully shutdown the server...");
    
    // Create app state
    let state = Arc::new(AppState {
        file_path: config.file_path,
    });
    
    // Create router
    let app = create_router(state);
    
    // Bind TCP listener
    let listener = TcpListener::bind(addr).await?;
    
    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    // Spawn server task
    let server_handle = tokio::spawn(async move {
        let serve_future = axum::serve(listener, app);
        
        // Wait for either server completion or shutdown signal
        tokio::select! {
            result = serve_future => {
                if let Err(err) = result {
                    eprintln!("Server error: {}", err);
                }
            },
            _ = shutdown_rx => {
                println!("Shutting down server...");
            }
        }
    });
    
    // Wait for shutdown signal
    wait_for_shutdown().await?;
    
    // Send shutdown signal to server task
    let _ = shutdown_tx.send(());
    
    // Wait for server task to complete
    server_handle.await?;
    
    println!("Server gracefully shutdown.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let config = ServerConfig::parse();
    
    // Run the server
    run_server(config).await
}
