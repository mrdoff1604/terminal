use std::io::{stdin, stdout, Write};

use futures_util::StreamExt;
use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{info, error, debug};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("ws_client=info")
        .init();

    let url = "ws://localhost:8080/ws";
    info!("Connecting to WebSocket server at {}", url);

    // Connect to the WebSocket server
    let (ws_stream, _) = match connect_async(url).await {
        Ok((stream, response)) => {
            info!("Connected to server! Response: {:?}", response.status());
            (stream, response)
        },
        Err(e) => {
            error!("Failed to connect to WebSocket server: {}", e);
            return;
        },
    };

    info!("WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to read messages from the server
    let read_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received from server: {}", text);
                },
                Ok(Message::Binary(bin)) => {
                    debug!("Received binary message from server: {:?}", bin);
                },
                Ok(Message::Ping(_ping)) => {
                    debug!("Received ping from server");
                },
                Ok(Message::Pong(_)) => {
                    debug!("Received pong from server");
                },
                Ok(Message::Close(frame)) => {
                    if let Some(frame) = frame {
                        info!("Received close frame from server: code={}, reason={}", frame.code, frame.reason);
                    } else {
                        info!("Received close frame from server");
                    }
                    break;
                },
                Ok(Message::Frame(frame)) => {
                    debug!("Received raw frame from server: {:?}", frame);
                },
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                },
            }
        }
    });

    // Spawn a task to write messages to the server
    let write_task = tokio::spawn(async move {
        loop {
            // Read input from stdin
            let mut input = String::new();
            print!("Enter message: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            
            let input = input.trim();
            
            if input == "/quit" {
                info!("Closing connection...");
                if let Err(e) = write.send(Message::Close(None)).await {
                    error!("Failed to send close message: {}", e);
                }
                break;
            }
            
            // Send the message to the server
            if let Err(e) = write.send(Message::Text(input.to_string())).await {
                error!("Failed to send message: {}", e);
                break;
            }
            
            info!("Sent message: {}", input);
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = read_task => info!("Read task completed"),
        _ = write_task => info!("Write task completed"),
    }

    info!("WebSocket client exiting");
}
