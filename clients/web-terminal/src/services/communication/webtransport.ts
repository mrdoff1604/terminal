/**
 * WebTransport Terminal Communication Implementation
 */

import { APP_CONFIG } from '../../config/appConfig';
import { TerminalEventType, TerminalEventHandler, TerminalCommunication } from '../terminalCommunication';

const WEBTRANSPORT_SERVER_URL = APP_CONFIG.WEBTRANSPORT_SERVER.URL;

// WebTransport implementation
export class WebTransportCommunication implements TerminalCommunication {
  private transport: WebTransport | null = null;
  private writer: WritableStreamDefaultWriter<string> | null = null;
  private reader: ReadableStreamDefaultReader<string> | null = null;
  private eventHandlers: Map<TerminalEventType, Set<TerminalEventHandler>> = new Map();
  private url: string;
  private isReading: boolean = false;

  constructor(sessionId: string) {
    // WebTransport requires a full URL with https scheme
    // We'll use the WebTransport server URL directly, bypassing the proxy
    // This avoids potential proxy issues with WebTransport
    this.url = `${WEBTRANSPORT_SERVER_URL}/webtransport/${sessionId}`;
  }

  async connect(): Promise<void> {
    try {
      // Check if WebTransport is supported
      if (!('WebTransport' in window)) {
        throw new Error('WebTransport is not supported in this browser');
      }

      console.log(`Attempting to connect to WebTransport server at: ${this.url}`);
      
      // Create WebTransport connection
      this.transport = new WebTransport(this.url);
      
      // Wait for the connection to be established
      console.log('Waiting for WebTransport connection to be ready...');
      await this.transport.ready;
      console.log('WebTransport connection established successfully');
      this.emit('open', {});
      
      // Create a bidirectional stream
      console.log('Creating WebTransport bidirectional stream...');
      const stream = await this.transport.createBidirectionalStream();
      console.log('WebTransport bidirectional stream created');
      
      // Set up writer
      this.writer = stream.writable.getWriter();
      console.log('WebTransport writer created');
      
      // Set up reader
      this.reader = stream.readable
        .pipeThrough(new TextDecoderStream())
        .getReader();
      console.log('WebTransport reader created');
      
      // Start reading messages
      this.readMessages();
      console.log('WebTransport message reading started');
    } catch (error: any) {
      console.error('WebTransport connection error:', error);
      console.error('Error name:', error.name);
      console.error('Error message:', error.message);
      console.error('Error stack:', error.stack);
      this.emit('error', error);
    }
  }

  disconnect(): void {
    // Close writer
    if (this.writer) {
      this.writer.close();
      this.writer = null;
    }
    
    // Close reader
    if (this.reader) {
      this.reader.cancel();
      this.reader = null;
    }
    
    // Close transport
    if (this.transport) {
      this.transport.close();
      this.transport = null;
    }
    
    this.isReading = false;
    // Clear all event handlers
    this.eventHandlers.clear();
    
    this.emit('close', {});
  }

  async send(data: string): Promise<void> {
    if (this.writer && this.transport?.ready) {
      try {
        await this.writer.write(data);
      } catch (error) {
        this.emit('error', error);
      }
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
    // WebTransport doesn't have a readyState property
    // Check if transport exists and we have active streams
    return this.transport !== null && this.writer !== null && this.reader !== null;
  }

  private async readMessages(): Promise<void> {
    if (this.isReading || !this.reader) return;
    
    this.isReading = true;
    
    try {
      while (true) {
        const { done, value } = await this.reader.read();
        if (done) break;
        this.emit('message', value);
      }
    } catch (error) {
      this.emit('error', error);
    } finally {
      this.isReading = false;
    }
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