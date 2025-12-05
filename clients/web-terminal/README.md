# Waylon Terminal - Web Client

A modern web-based terminal client built with React, TypeScript, and xterm.js, providing a seamless terminal experience in the browser.

## Project Overview

The Waylon Terminal web client is the frontend component of a web-based terminal application that allows users to create and manage terminal sessions through a browser interface. It provides a rich terminal experience with support for real-time communication, dynamic resizing, and multiple session management.

## System Architecture

The client follows a modern React architecture with clear separation of concerns:

```
┌───────────────────────────────────────────────────────────────────────────────────┐
│                                  Application Layer                                 │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │   App.tsx      │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  Component Layer                                  │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │    Header      │                              │
│                                  ├────────────────┘                              │
│                                  │    Terminal    │                              │
│                                  ├────────────────┘                              │
│                                  │  SessionPanel  │                              │
│                                  ├────────────────┘                              │
│                                  │  ...Other      │                              │
│                                  │  Components    │                              │
│                                  └────────────────┘                              │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  Service Layer                                   │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                  ┌────────────────┐                              │
│                                  │  Terminal API  │                              │
│                                  ├────────────────┘                              │
│                                  │ WebSocket Client│                              │
│                                  ├────────────────┘                              │
│                                  │ WebTransport Client │                           │
│                                  └────────────────┘                              │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## Core Features

### Terminal Experience
- **Full terminal emulation** using xterm.js
- **Dynamic resizing** - Resize terminals in real-time
- **Multiple session support** - Run and manage multiple terminals simultaneously
- **Fullscreen mode** - Immersive terminal experience

### Real-time Communication
- **WebSocket support** for reliable real-time communication
- **WebTransport support** for low-latency communication
- **Automatic protocol fallback** - Uses best available protocol

### User Interface
- **Responsive design** - Works on desktop and mobile devices
- **Modern UI** - Clean, intuitive interface built with React
- **Session management panel** - Easy session switching and management
- **Customizable themes** - Personalize the terminal appearance

## Technology Stack

### Core Technologies
- **React 19.2.1** - Modern UI framework
- **TypeScript 5.9.3** - Type-safe development
- **Vite 7.2.6** - Fast build tool and dev server
- **xterm.js 5.5.0** - Terminal emulation

### UI Components and Styling
- **Radix UI** - Accessible UI components
- **Tailwind CSS 3.4.18** - Utility-first CSS framework
- **lucide-react 0.555.0** - Icon library

### Communication
- **WebSocket API** - Real-time communication
- **WebTransport API** - Low-latency communication

## Project Structure

```
src/
├── components/        # React components
│   ├── Header.tsx     # Application header
│   ├── Terminal.tsx   # Terminal emulation component
│   └── SessionPanel.tsx # Session management panel
├── config/            # Application configuration
│   └── appConfig.ts   # API endpoints and settings
├── services/          # API and communication services
│   └── terminalApi.ts # Terminal API client
├── App.tsx            # Main application component
└── main.tsx           # Application entry point
```

## Quick Start

### Prerequisites
- Node.js 20 or higher
- pnpm (Package manager)

### Build and Run

1. **Install dependencies**:
   ```bash
   pnpm install
   ```

2. **Start development server**:
   ```bash
   pnpm run dev
   ```

3. **Build for production**:
   ```bash
   pnpm run build
   ```

4. **Preview production build**:
   ```bash
   pnpm run preview
   ```

### Configuration

The application can be configured through the `appConfig.ts` file located in `src/config/`.

Key configuration options:
- `API_BASE_URL` - Base URL for REST API requests
- `WS_BASE_URL` - Base URL for WebSocket connections
- `APP_NAME` - Application name displayed in the UI

## Development

### Code Style
- Follow TypeScript best practices
- Use ESLint for code linting
- Follow React hooks best practices

### Testing
- Use React Testing Library for component testing
- Write comprehensive tests for new functionality

### Building
- The application is built using Vite
- Output directory: `dist/`
- Source maps are generated for debugging

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
