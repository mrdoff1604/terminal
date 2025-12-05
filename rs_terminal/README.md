# Waylon Terminal - Rust Backend

A high-performance terminal server implementation built with Rust, providing low-latency terminal access through WebSocket and WebTransport protocols.

## Project Overview

The Waylon Terminal Rust backend is a high-performance implementation of a terminal server that allows users to create and manage terminal sessions through a web interface. It provides RESTful APIs for session management and real-time communication via WebSocket and WebTransport protocols, with a focus on performance and reliability.

## System Architecture

The system follows a modern Rust architecture with clear separation of concerns:

```
┌───────────────────────────────────────────────────────────────────────────────────┐
│                                    API Layer                                     │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  REST Routes   │                              │
│                                  ├────────────────┘                              │
│                                  │  WebSocket     │                              │
│                                  ├────────────────┘                              │
│                                  │  WebTransport  │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                 Application Layer                                 │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │   Services     │                              │
│                                  ├────────────────┘                              │
│                                  │   Processes    │                              │
│                                  ├────────────────┘                              │
│                                  │   Handlers     │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  Domain Layer                                    │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  Models        │                              │
│                                  ├────────────────┘                              │
│                                  │  Repository    │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                               Infrastructure Layer                               │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  Config        │                              │
│                                  ├────────────────┘                              │
│                                  │  Logging       │                              │
│                                  ├────────────────┘                              │
│                                  │  PTY           │                              │
│                                  └────────────────┘                              │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## Core Features

### Session Management
- Create, retrieve, update, and delete terminal sessions
- Support for multiple terminal sessions per user
- Session lifecycle management
- Session activity tracking

### Terminal Operations
- Real-time terminal communication
- Terminal resizing
- Shell type selection
- Working directory configuration

### Communication Protocols
- WebSocket support for reliable real-time communication
- WebTransport support for low-latency communication
- Efficient message handling

### Process Management
- Portable PTY (Pseudo-Terminal) implementation
- Cross-platform terminal support
- Process lifecycle management
- Resource-efficient design

## Technology Stack

### Core Technologies
- **Rust 2024 Edition** - Systems programming language
- **Tokio 1.48.0** - Asynchronous runtime
- **Axum 0.8.7** - Web framework
- **wtransport 0.6.1** - WebTransport implementation
- **tokio-tungstenite 0.28.0** - WebSocket implementation

### Terminal Emulation
- **portable-pty 0.9.0** - Cross-platform PTY implementation

### Serialization
- **Serde 1.0.228** - Serialization/deserialization

### Logging and Monitoring
- **log4rs 1.3.0** - Logging framework
- **anyhow 1.0.100** - Error handling

### Configuration
- **config 0.15.0** - Configuration management

## Project Structure

```
src/
├── main.rs                    # Application entry point
├── config/                    # Configuration management
├── handlers/                  # HTTP request handlers
├── models/                    # Domain models
├── repository/                # Data access layer
├── services/                  # Business logic services
├── terminal/                  # Terminal process management
├── websocket/                 # WebSocket implementation
└── webtransport/              # WebTransport implementation
```

## Quick Start

### Prerequisites
- Rust 1.80 or higher
- Cargo (Rust package manager)

### Build and Run

1. **Build the project**:
   ```bash
   cargo build
   ```

2. **Run the application**:
   ```bash
   cargo run
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Build with release optimizations**:
   ```bash
   cargo build --release
   ```

### Configuration

The application can be configured through the `config.toml` file located in the project root.

Key configuration options:
- `server.port` - Server port
- `terminal.default_shell` - Default shell type
- `logging.level` - Logging level
- `websocket.enabled` - Enable WebSocket support
- `webtransport.enabled` - Enable WebTransport support

## Development

### Code Style
- Follow Rust best practices
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for code linting

### Performance Focus
- Use async/await for non-blocking operations
- Optimize memory usage
- Minimize allocations
- Use efficient data structures

### Testing
- Write unit tests for core functionality
- Write integration tests for API endpoints
- Test with multiple shell environments

## Core Components

### Terminal Session
Represents a terminal session with all its lifecycle management capabilities.

### PTY Manager
Handles the creation and management of PTY processes.

### WebSocket Server
Provides real-time communication through WebSocket protocol.

### WebTransport Server
Provides low-latency communication through WebTransport protocol.

### Session Repository
Manages the persistence and retrieval of terminal sessions.

## API Endpoints

### RESTful APIs

#### Session Management
- `POST /api/sessions` - Create a new terminal session
- `GET /api/sessions` - Get all terminal sessions
- `GET /api/sessions/{id}` - Get a specific terminal session
- `DELETE /api/sessions/{id}` - Delete a terminal session
- `POST /api/sessions/{id}/resize` - Resize a terminal session

### WebSocket API
- `ws://localhost:8080/ws` - WebSocket endpoint for terminal communication

### WebTransport API
- `https://localhost:8082` - WebTransport endpoint for terminal communication

## Performance Features

- **Non-blocking I/O** - Uses Tokio for asynchronous operations
- **Efficient message handling** - Minimizes overhead for real-time communication
- **Memory efficient** - Designed to use minimal memory
- **High concurrency** - Handles multiple connections efficiently
- **Fast startup time** - Quick initialization for rapid deployment

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
