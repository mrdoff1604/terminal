/**
 * WebSocket Terminal Communication Implementation
 */

import { APP_CONFIG } from '../../config/appConfig';
import { TerminalEventType, TerminalEventHandler, TerminalCommunication } from '../terminalCommunication';

const WS_SERVER_URL = APP_CONFIG.WS_SERVER.URL;

// WebSocket implementation
export class WebSocketCommunication implements TerminalCommunication {
  private ws: WebSocket | null = null;
  private eventHandlers: Map<TerminalEventType, Set<TerminalEventHandler>> = new Map();
  private url: string;

  constructor(sessionId: string) {
    this.url = `${WS_SERVER_URL}/ws/${sessionId}`;
  }

  connect(): void {
    try {
      this.ws = new WebSocket(this.url);
      
      this.ws.onopen = (event) => {
        this.emit('open', event);
      };
      
      this.ws.onmessage = (event) => {
        this.emit('message', event.data);
      };
      
      this.ws.onclose = (event) => {
        this.emit('close', event);
      };
      
      this.ws.onerror = (event) => {
        this.emit('error', event);
      };
    } catch (error) {
      this.emit('error', error);
    }
  }

  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    // Clear all event handlers
    this.eventHandlers.clear();
  }

  send(data: string): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(data);
    }
  }

  on(event: TerminalEventType, handler: TerminalEventHandler): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)?.add(handler);
  }

  off(event: TerminalEventType, handler: TerminalEventHandler): void {
    this.eventHandlers.get(event)?.delete(handler);
  }

  isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  private emit(event: TerminalEventType, data: any): void {
    this.eventHandlers.get(event)?.forEach(handler => {
      try {
        handler(data);
      } catch (error) {
        console.error(`Error in ${event} handler:`, error);
      }
    });
  }
}