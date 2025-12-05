/**
 * Terminal Communication Service
 * Handles WebSocket and WebTransport communication for terminal sessions
 * Entry point for terminal communication, exports interfaces and factory functions
 */

// Communication event types
export type TerminalEventType = 'open' | 'message' | 'close' | 'error';

// Communication event handler type
export type TerminalEventHandler = (event: any) => void;

// Terminal communication interface
export interface TerminalCommunication {
  connect(): void;
  disconnect(): void;
  send(data: string): void;
  on(event: TerminalEventType, handler: TerminalEventHandler): void;
  off(event: TerminalEventType, handler: TerminalEventHandler): void;
  isConnected(): boolean;
}

// Import implementations from separate files
import { WebSocketCommunication } from './communication/websocket';
import { WebTransportCommunication } from './communication/webtransport';

// Factory function to create terminal communication instance
export const createTerminalCommunication = (sessionId: string, protocol: 'websocket' | 'webtransport' = 'websocket'): TerminalCommunication => {
  if (protocol === 'webtransport') {
    return new WebTransportCommunication(sessionId);
  } else {
    return new WebSocketCommunication(sessionId);
  }
};

// Check if WebTransport is supported
export const isWebTransportSupported = (): boolean => {
  return 'WebTransport' in window;
};
