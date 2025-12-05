import React from 'react';
import { createSession, resizeTerminal, interruptTerminal, terminateSession } from './terminalApi';
import { createTerminalCommunication, TerminalCommunication, isWebTransportSupported } from './terminalCommunication';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { WebglAddon } from '@xterm/addon-webgl';

// 终端服务类，处理终端相关的业务逻辑
export class TerminalService {
  private communication: TerminalCommunication | null = null;
  private sessionId: string = '';
  private shellType: string = '';
  private protocol: 'websocket' | 'webtransport' | 'auto';
  private onConnectionStatusChange?: (connected: boolean, sessionInfo?: {
    sessionId: string;
    shellType: string;
    terminalSize: { columns: number; rows: number };
  }) => void;

  constructor(protocol: 'websocket' | 'webtransport' | 'auto' = 'auto', onConnectionStatusChange?: (connected: boolean, sessionInfo?: {
    sessionId: string;
    shellType: string;
    terminalSize: { columns: number; rows: number };
  }) => void) {
    this.protocol = protocol;
    this.onConnectionStatusChange = onConnectionStatusChange;
  }

  // 连接终端
  async connect(terminal: Terminal, columns: number, rows: number): Promise<void> {
    try {
      console.log('🔄 Starting terminal connection process...');
      
      // Get or generate user ID
      let userId = localStorage.getItem('terminal_user_id');
      
      // Check if userId exists and has correct format (usr_ + 12 lowercase hex chars)
      const isValidUserId = userId && userId.startsWith('usr_') && userId.length === 16 && /^usr_[a-f0-9]{12}$/.test(userId);
      
      if (!userId || !isValidUserId) {
        // Generate user ID in format required by backend: usr_ + 12 lowercase hex characters
        const hexChars = 'abcdef0123456789';
        let hexId = '';
        for (let i = 0; i < 12; i++) {
          hexId += hexChars.charAt(Math.floor(Math.random() * hexChars.length));
        }
        userId = 'usr_' + hexId.toLowerCase();
        localStorage.setItem('terminal_user_id', userId);
        
        if (!isValidUserId && userId) {
          console.log('🔄 Replaced invalid userId format with new valid format:', userId);
        }
      }
      
      // 1. Create new session via API
      console.log('📡 Creating new session via API...');
      
      const sessionResponse = await createSession(userId, 'Terminal Session', undefined, columns, rows);
      const newSessionId = sessionResponse.id; // 后端返回的是id字段，不是sessionId
      const shellType = sessionResponse.shellType; // 后端直接返回shellType字段，不是在configuration中
      this.shellType = shellType;
      
      // 使用实际的终端尺寸数据
      const terminalSize = { columns, rows };
      
      console.log('✅ Session created:', newSessionId, 'Shell type:', shellType, 'Terminal size:', `${terminalSize.columns}×${terminalSize.rows}`);
      this.sessionId = newSessionId;
      
      // 2. Try to establish communication connection (one-to-one binding)
      console.log('🌐 Attempting to establish communication connection for session...');
      
      try {
        // Determine which protocol to use
        let selectedProtocol: 'websocket' | 'webtransport';
        
        if (this.protocol === 'auto') {
          // Auto-detect: use WebTransport if supported, otherwise WebSocket
          selectedProtocol = isWebTransportSupported() ? 'webtransport' : 'websocket';
        } else {
          // Use the protocol specified by the user
          selectedProtocol = this.protocol;
        }
        
        console.log(`📡 Using communication protocol: ${selectedProtocol}`);
        
        // Create communication instance
        this.communication = createTerminalCommunication(newSessionId, selectedProtocol);
        
        // Set up event handlers
        this.communication.on('open', () => {
          console.log('✅ Communication connection established successfully');
          
          // Configure terminal parameters after connection is successful
          this.configureTerminalForShell(terminal, shellType);
          
          terminal.write('Terminal ready\r\n');
          
          // 传递会话信息给父组件
          this.onConnectionStatusChange?.(true, {
            sessionId: newSessionId,
            shellType: shellType,
            terminalSize: terminalSize // 使用尺寸对象
          });
          
          // After successful connection, session and communication have established one-to-one relationship
          console.log(`🔗 Session ${newSessionId} ↔ ${selectedProtocol} connection established`);
        });
        
        this.communication.on('message', (data) => {
          console.log('📨 Received terminal output:', data);
          
          // Communication is only used for command line output, display directly
          if (typeof data === 'string') {
            // xterm.js is specifically designed to handle terminal escape sequences, no manual escaping needed
            // Write data directly, let xterm.js handle all ANSI escape sequences
            terminal.write(data);
          }
        });
        
        this.communication.on('close', (event) => {
          console.log('🔌 Communication connection closed');
          console.log(`📊 Close event:`, event);
          
          this.onConnectionStatusChange?.(false);
          terminal.writeln('\r\nCommunication connection closed');
          
          // When connection closes, session should also be terminated (one-to-one relationship)
          if (this.sessionId) {
            console.log(`🛑 Terminating session ${this.sessionId} due to communication closure`);
            this.terminate('COMMUNICATION_CLOSED');
          }
        });
        
        this.communication.on('error', (error) => {
          console.error('❌ Communication connection error:', error);
          terminal.writeln('❌ Communication connection error');
          
          this.onConnectionStatusChange?.(false);
          
          // When communication error occurs, session should also be terminated (one-to-one relationship)
          if (this.sessionId) {
            console.log(`🛑 Terminating session ${this.sessionId} due to communication error`);
            this.terminate('COMMUNICATION_ERROR');
          }
        });
        
        // Connect to the server
        this.communication.connect();
      } catch (error) {
        console.warn('⚠️ Communication connection failed, using fallback mode:', error);
        
        // Configure terminal parameters even without communication
        this.configureTerminalForShell(terminal, shellType);
        
        // 直接使用尺寸对象调整xterm.js
        if (terminalSize.columns && terminalSize.rows) {
          terminal.resize(terminalSize.columns, terminalSize.rows);
        }
        
        terminal.write('$ ');
        
        // 传递会话信息给父组件
        this.onConnectionStatusChange?.(true, {
          sessionId: newSessionId,
          shellType: shellType,
          terminalSize: terminalSize // 使用尺寸对象
        });
        
        console.log(`✅ Session ${newSessionId} created (fallback mode)`);
      }
      
    } catch (error) {
      console.error('❌ Failed to connect terminal:', error);
      terminal.writeln('❌ Failed to connect terminal');
      
      this.onConnectionStatusChange?.(false);
    }
  }

  // 断开连接
  disconnect(): void {
    if (this.sessionId) {
      this.terminate('USER_DISCONNECTED');
    } else if (this.communication) {
      this.communication.disconnect();
    }
  }

  // 发送数据
  send(data: string): void {
    if (this.communication && this.communication.isConnected()) {
      // Do not perform any local echo, let backend handle all output
      // Send all input to backend, backend is responsible for complete command processing and echo
      this.communication.send(data);
    }
  }

  // 调整终端大小
  async resize(columns: number, rows: number): Promise<void> {
    if (!this.sessionId) {
      console.warn('⚠️ No active session to resize');
      return;
    }
    
    try {
      console.log(`📐 Resizing terminal to ${columns}x${rows}`);
      await resizeTerminal(this.sessionId, columns, rows);
      
      // 更新父组件状态
      this.onConnectionStatusChange?.(true, {
        sessionId: this.sessionId,
        shellType: this.shellType,
        terminalSize: { columns, rows }
      });
      
      console.log('✅ Terminal resized successfully');
    } catch (error) {
      console.error('❌ Failed to resize terminal:', error);
    }
  }

  // 中断终端（发送Ctrl+C信号）
  async interrupt(): Promise<void> {
    if (!this.sessionId) {
      console.warn('⚠️ No active session to interrupt');
      return;
    }
    
    try {
      console.log('⏹️ Sending interrupt signal to terminal');
      await interruptTerminal(this.sessionId);
      
      console.log('✅ Terminal interrupted successfully');
    } catch (error) {
      console.error('❌ Failed to interrupt terminal:', error);
    }
  }

  // 终止会话
  async terminate(reason?: string): Promise<void> {
    if (!this.sessionId) {
      console.warn('⚠️ No active session to terminate');
      return;
    }
    
    try {
      console.log(`🛑 Terminating session: ${reason || 'USER_REQUESTED'}`);
      
      // First close communication connection
      if (this.communication) {
        this.communication.disconnect();
        console.log('🔌 Communication connection closed');
      }
      
      // Then terminate session
      await terminateSession(this.sessionId, reason);
      console.log('✅ Session terminated successfully');
      
      // Reset state
      this.sessionId = '';
      this.onConnectionStatusChange?.(false);
      
    } catch (error) {
      console.error('❌ Failed to terminate session:', error);
      
      // Even if API call fails, ensure communication is closed
      if (this.communication) {
        this.communication.disconnect();
      }
    }
  }

  // 动态配置xterm.js参数基于shell类型
  configureTerminalForShell(terminal: Terminal, shellType: string | undefined): void {
    // Handle undefined or empty values
    if (!shellType) {
      console.warn('⚠️ Shell type is undefined or empty, using auto-detection');
      shellType = 'AUTO';
    }
    
    console.log(`⚙️ Configuring terminal for shell type: ${shellType}`);
    
    // Set different xterm.js configurations based on shell type
    switch (shellType.toUpperCase()) {
      case 'WINDOWS_CMD':
      case 'WINDOWS_POWERSHELL':
        // Windows environment: enable Windows mode, handle carriage return correctly
        terminal.options.windowsMode = true;
        terminal.options.convertEol = true; // Convert \n to \r\n
        break;
        
      case 'UNIX':
        // Unix/Linux environment: use Unix-style line endings
        terminal.options.windowsMode = false;
        terminal.options.convertEol = false; // Keep \n unchanged
        break;
        
      case 'AUTO':
      default:
        // Auto-detection: determine based on browser environment
        const isWindows = navigator.userAgent.includes('Windows');
        terminal.options.windowsMode = isWindows;
        terminal.options.convertEol = isWindows;
        break;
    }
    
    // Refresh terminal configuration
    terminal.refresh(0, terminal.rows - 1);
  }

  // 获取当前会话ID
  getSessionId(): string {
    return this.sessionId;
  }

  // 获取当前shell类型
  getShellType(): string {
    return this.shellType;
  }
}

// 终端初始化配置
export const initializeTerminal = (terminalRef: React.RefObject<HTMLDivElement | null>, onData: (data: string) => void): {
  terminal: Terminal;
  fitAddon: FitAddon;
} => {
  const terminal = new Terminal({
    // Basic configuration
    fontSize: 14,
    fontFamily: 'Consolas, "Courier New", monospace',
    theme: {
      background: '#1e1e1e',
      foreground: '#cccccc',
      cursor: '#ffffff'
    }
    // 不设置固定尺寸，让终端自动适应容器大小
    // Do not add any special configuration, let xterm.js handle all characters in default way
  });

  // Create and install addons
  const fitAddon = new FitAddon();
  const webLinksAddon = new WebLinksAddon();
  const webglAddon = new WebglAddon();

  terminal.loadAddon(fitAddon);
  terminal.loadAddon(webLinksAddon);
  terminal.loadAddon(webglAddon);

  // Mount to DOM
  if (terminalRef.current) {
    terminal.open(terminalRef.current);
  }

  // Listen for keyboard input - using the simplest processing method
  terminal.onData(onData);

  return { terminal, fitAddon };
};

// 终端工具函数
export const terminalUtils = {
  // 处理窗口大小改变
  handleWindowResize: (fitAddon: FitAddon) => {
    fitAddon.fit();
  },

  // 显示欢迎消息
  showWelcomeMessage: (terminal: Terminal) => {
    terminal.writeln('Click the "Connect" button to start a session');
    terminal.write('$ ');
  }
};
