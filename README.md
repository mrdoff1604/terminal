# Waylon Terminal - Web-Based Terminal Application

A modern, multi-protocol web terminal application with support for real-time communication, built with cutting-edge technologies across multiple platforms.

## 🚀 Core Business Features

### Terminal Session Management
- **Create and manage** terminal sessions through a web interface
- **Multi-session support** - Run multiple terminals simultaneously
- **Session persistence** - Resume sessions across browser refreshes
- **User-based session isolation** - Secure session management per user

### Real-time Communication
- **WebSocket support** for reliable real-time communication
- **WebTransport support** for low-latency communication (future-proof)
- **Automatic protocol fallback** - Uses best available protocol

### Terminal Features
- **Full terminal emulation** using xterm.js
- **Dynamic resizing** - Resize terminals in real-time
- **Multiple shell support** - Configure different shell types
- **Customizable working directories** - Start terminals in any directory
- **Environment variable support** - Configure shell environments

### User Experience
- **Responsive design** - Works on desktop and mobile devices
- **Fullscreen mode** - Immersive terminal experience
- **Modern UI** - Clean, intuitive interface built with React
- **Session management panel** - Easy session switching and management

## 📸 Waylon Terminal Demo

![Waylon Terminal Demo](assets/kt-terminal-demo3.png)

## 🛠️ Technical Architecture

### Frontend Implementation
**Location**: `clients/web-terminal`

A modern React-based web client built with TypeScript and xterm.js, providing a seamless terminal experience in the browser.

**Detailed Documentation**: [Web Terminal README](clients/web-terminal/README.md)

### Backend Implementations

#### Kotlin Implementation (Primary)
**Location**: `kt-terminal`

A modern terminal server built with Kotlin, following DDD+Kotlin+SOLID principles, with support for WebSocket and WebTransport protocols.

**Detailed Documentation**: [Kotlin Terminal README](kt-terminal/README.md)

#### Rust Implementation
**Location**: `rs_terminal`

A high-performance terminal server built with Rust, providing low-latency terminal access through WebSocket and WebTransport protocols.

**Detailed Documentation**: [Rust Terminal README](rs_terminal/README.md)

## 🌟 Technology Highlights

### Multi-Protocol Support
- WebSocket for reliable communication
- WebTransport for next-generation low-latency communication
- Automatic protocol negotiation and fallback

### Domain-Driven Design
- Clear bounded contexts
- Rich domain models
- Repository pattern
- Use case-driven architecture
- Clean separation of concerns

### SOLID Design Principles
- Single Responsibility Principle
- Open/Closed Principle
- Liskov Substitution Principle
- Interface Segregation Principle
- Dependency Inversion Principle

### Modern Language Features
- Kotlin coroutines for async programming
- Rust's safety and performance
- TypeScript for type safety
- React hooks for component logic

### Cross-Platform Support
- Works on Windows, macOS, and Linux
- Multiple backend implementations for different use cases
- Responsive design for mobile and desktop



## 🚀 Getting Started

### Prerequisites
- **Node.js 20+** - For frontend development
- **pnpm** - Package manager for frontend
- **Java 21+** - For Kotlin backend
- **Gradle 8.5+** - Build tool for Kotlin backend
- **Rust 1.80+** - For Rust backend (optional)

### Frontend Development
```bash
# Install dependencies
cd clients/web-terminal
pnpm install

# Start development server
pnpm run dev
```

### Kotlin Backend Development
```bash
# Build the project
cd kt-terminal
./gradlew build

# Run the application
./gradlew run

# Run tests
./gradlew test
```

### Rust Backend Development (Optional)
```bash
# Build the project
cd rs_terminal
cargo build

# Run the application
cargo run

# Run tests
cargo test
```

## 📱 Usage

1. **Start the backend server**
   ```bash
   cd kt-terminal
   ./gradlew run
   ```

2. **Start the frontend development server**
   ```bash
   cd clients/web-terminal
   pnpm run dev
   ```

3. **Open your browser**
   - Navigate to `http://localhost:3000`
   - Create a new terminal session
   - Start using the terminal!

## 🔧 Configuration

### Frontend Configuration
- Located in `clients/web-terminal/src/config/appConfig.ts`
- Configures API endpoints, WebSocket URLs, and application settings

### Backend Configuration
- Located in `kt-terminal/src/main/resources/application.conf`
- Configures server port, shell settings, session timeout, and more

## 📋 Core API Endpoints

### Session Management
- `POST /api/sessions` - Create a new terminal session
- `GET /api/sessions` - Get all terminal sessions
- `GET /api/sessions/{id}` - Get a specific terminal session
- `DELETE /api/sessions/{id}` - Delete a terminal session
- `POST /api/sessions/{id}/resize` - Resize a terminal session

### Real-time Communication
- `ws://localhost:8080/ws` - WebSocket endpoint
- `https://localhost:8082` - WebTransport endpoint

## 🎯 Key Benefits

### For Developers
- Modern, type-safe development environment
- Clear separation of concerns
- Comprehensive documentation
- Test-driven development support
- Easy to extend and modify

### For Users
- Responsive, intuitive interface
- Low-latency terminal experience
- Secure session management
- Multi-session support
- Cross-platform compatibility

### For Organizations
- Scalable architecture
- Multiple backend options
- Easy deployment
- Comprehensive logging and monitoring
- Secure by design

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Contribution Guidelines
1. Follow the existing code style
2. Write comprehensive tests
3. Update documentation as needed
4. Create small, focused pull requests
5. Follow the project's architecture

## 📞 Contact

For questions or feedback, please open an issue on the GitHub repository.

## 📚 Additional Resources

- [React Documentation](https://react.dev/)
- [Kotlin Documentation](https://kotlinlang.org/docs/home.html)
- [Ktor Documentation](https://ktor.io/docs/welcome.html)
- [Rust Documentation](https://www.rust-lang.org/learn)
- [xterm.js Documentation](https://xtermjs.org/docs/)
- [WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API)
- [WebTransport API](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API)

---

**Waylon Terminal** - Empowering developers with a modern, high-performance web terminal experience.

⭐ If you find this project useful, please give it a star!