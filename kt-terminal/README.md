# KT Terminal - Server Side Implementation

A modern terminal server implementation built with Kotlin, following Domain-Driven Design (DDD) principles and SOLID design principles.

## Project Overview

KT Terminal is the backend component of a web-based terminal application that allows users to create and manage terminal sessions through a browser interface. The server provides RESTful APIs for session management and real-time communication via WebSocket and WebTransport protocols.

## System Architecture

The system follows a clean DDD architecture with clear separation of concerns:

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
│                                  │   Use Cases    │                              │
│                                  ├────────────────┘                              │
│                                  │   Services     │                              │
│                                  ├────────────────┘                              │
│                                  │   Processes    │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  Domain Layer                                    │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  Aggregates    │                              │
│                                  ├────────────────┘                              │
│                                  │   Entities     │                              │
│                                  ├────────────────┘                              │
│                                  │ Value Objects  │                              │
│                                  ├────────────────┘                              │
│                                  │   Repositories │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                               Infrastructure Layer                               │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  Data Storage  │                              │
│                                  ├────────────────┘                              │
│                                  │     Config     │                              │
│                                  ├────────────────┘                              │
│                                  │    Protocols   │                              │
│                                  └────────────────┘                              │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## Core Features

### Session Management
- Create, retrieve, update, and delete terminal sessions
- Support for multiple terminal sessions per user
- Session timeout management
- Session activity tracking

### Terminal Operations
- Real-time terminal communication
- Terminal resizing
- Shell type selection
- Working directory configuration
- Environment variable support

### Communication Protocols
- WebSocket support for real-time communication
- WebTransport support for low-latency communication
- Automatic protocol fallback

### Process Management
- PTY (Pseudo-Terminal) process management
- Cross-platform terminal support
- Process lifecycle management

## Technology Stack

### Core Technologies
- **Kotlin** - Modern JVM language
- **Ktor** - Asynchronous web framework
- **Koin** - Dependency injection framework
- **Coroutines** - Asynchronous programming
- **PTY4J** - Pseudo-terminal implementation

### Serialization
- **Kotlinx Serialization** - JSON serialization

### Testing
- **JUnit 5** - Unit testing
- **Ktor Test** - Integration testing

## Project Structure

```
src/main/kotlin/dev/waylon/terminal/
├── boundedcontexts/
│   └── terminalsession/
│       ├── application/
│       │   ├── process/           # Process management interfaces
│       │   ├── service/           # Application services
│       │   └── useCase/           # Use cases (business logic)
│       ├── domain/                # Domain model
│       │   ├── model/             # Domain models
│       │   ├── TerminalSession.kt # Aggregate root
│       │   └── ...
│       └── infrastructure/        # Infrastructure implementations
│           ├── config/            # Routing and module config
│           ├── dto/               # Data Transfer Objects
│           ├── protocol/          # Protocol implementations
│           ├── repository/        # Repository implementations
│           └── service/           # Infrastructure services
└── infrastructure/                # Shared infrastructure
    └── config/                    # Application configuration
```

## Quick Start

### Prerequisites
- Java 17 or higher
- Gradle 7.5 or higher
- Kotlin 1.8 or higher

### Build and Run

1. **Build the project**:
   ```bash
   ./gradlew build
   ```

2. **Run the application**:
   ```bash
   ./gradlew run
   ```

3. **Run tests**:
   ```bash
   ./gradlew test
   ```

### Configuration

The application can be configured through the `application.conf` file located in `src/main/resources/`.

Key configuration options:
- `server.port` - Server port
- `terminal.default-shell-type` - Default shell type
- `terminal.session-timeout-ms` - Session timeout in milliseconds
- `terminal.shells` - Shell configuration

## API Documentation

### RESTful APIs

#### Session Management
- `POST /api/sessions` - Create a new terminal session
- `GET /api/sessions` - Get all terminal sessions
- `GET /api/sessions/{id}` - Get a specific terminal session
- `DELETE /api/sessions/{id}` - Delete a terminal session
- `POST /api/sessions/{id}/resize` - Resize a terminal session

#### Request Examples

**Create Session**:
```bash
curl -X POST http://localhost:8080/api/sessions \
  -H "Content-Type: application/json" \
  -d '{"userId": "user123", "title": "My Terminal", "columns": 80, "rows": 24}'
```

**Resize Terminal**:
```bash
curl -X POST http://localhost:8080/api/sessions/{sessionId}/resize \
  -H "Content-Type: application/json" \
  -d '{"columns": 120, "rows": 40}'
```

### WebSocket API

- `ws://localhost:8080/ws` - WebSocket endpoint for terminal communication

### WebTransport API

- `https://localhost:8082` - WebTransport endpoint for terminal communication

## Design Principles

### DDD (Domain-Driven Design)
- Clear bounded context definition
- Rich domain model with behavior
- Repository pattern for data access
- Use case layer for business logic

### SOLID Principles
- **Single Responsibility** - Each class has a single responsibility
- **Open/Closed** - Extensible without modification
- **Liskov Substitution** - Subtypes can replace their base types
- **Interface Segregation** - Clients depend only on what they need
- **Dependency Inversion** - Depend on abstractions, not concretions

### Clean Code
- Meaningful naming
- Proper documentation (KDoc)
- Consistent formatting
- Comprehensive testing

## Core Domain Model

### TerminalSession (Aggregate Root)
Represents a terminal session with all its lifecycle management capabilities.

### TerminalSize (Value Object)
Represents the dimensions of a terminal window.

### TerminalSessionStatus (Enumeration)
Defines the possible states of a terminal session.

### TerminalConfig (Domain Model)
Represents the configuration for terminal sessions and shells.

## Use Cases

### Session Management
- `CreateTerminalSessionUseCase` - Creates a new terminal session
- `GetAllTerminalSessionsUseCase` - Retrieves all terminal sessions
- `GetTerminalSessionByIdUseCase` - Retrieves a terminal session by ID
- `ResizeTerminalUseCase` - Resizes a terminal session
- `TerminateTerminalSessionUseCase` - Terminates a terminal session

## Contribution Guidelines

1. **Code Style** - Follow the existing code style and conventions
2. **Testing** - Write comprehensive tests for new functionality
3. **Documentation** - Update documentation for changes
4. **Commit Messages** - Use descriptive commit messages
5. **Pull Requests** - Create small, focused pull requests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by modern terminal applications like iTerm2 and Windows Terminal
- Built with Kotlin and Ktor, following best practices
- Leveraging the power of coroutines for asynchronous programming

## Contact

For questions or feedback, please open an issue on the GitHub repository.

---

**KT Terminal** - A modern terminal server implementation
