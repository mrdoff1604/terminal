/**
 * Terminal Management API Service
 * Handles session management, size adjustment operations, separated from WebSocket
 */

import { APP_CONFIG } from '../config/appConfig';

const API_BASE_URL = `${APP_CONFIG.API_SERVER.URL}${APP_CONFIG.API_SERVER.BASE_PATH}/sessions`;

/**
 * Create new session
 */
export const createSession = async (userId: string, title?: string, workingDirectory?: string, columns?: number, rows?: number): Promise<{ id: string; userId: string; title: string | null; workingDirectory: string; shellType: string; status: string; terminalSize: { columns: number; rows: number }; createdAt: number; updatedAt: number }> => {
  try {
    const params = new URLSearchParams();
    params.append('userId', userId);
    if (title) params.append('title', title);
    if (workingDirectory) params.append('workingDirectory', workingDirectory);
    // 添加终端尺寸参数
    if (columns) params.append('columns', columns.toString());
    if (rows) params.append('rows', rows.toString());
    
    const url = `${API_BASE_URL}?${params.toString()}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    if (!response.ok) {
      throw new Error(`Failed to create session: ${response.statusText}`);
    }
    
    const responseData = await response.json();
    console.log('📡 API Response:', responseData);
    
    return responseData;
  } catch (error) {
    console.error('❌ Failed to create session:', error);
    throw error;
  }
};

/**
 * Resize terminal
 */
export const resizeTerminal = async (
  sessionId: string, 
  columns: number, 
  rows: number
): Promise<{ sessionId: string; terminalSize: { columns: number; rows: number }; status: string }> => {
  try {
    const params = new URLSearchParams();
    params.append('cols', columns.toString());
    params.append('rows', rows.toString());
    
    const url = `${API_BASE_URL}/${sessionId}/resize?${params.toString()}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    if (!response.ok) {
      throw new Error(`Failed to resize terminal: ${response.statusText}`);
    }
    
    return await response.json();
  } catch (error) {
    console.error('❌ Failed to resize terminal:', error);
    throw error;
  }
};

/**
 * Interrupt terminal (send Ctrl+C signal)
 */
export const interruptTerminal = async (
  sessionId: string
): Promise<{ sessionId: string; status: string }> => {
  try {
    const url = `${API_BASE_URL}/${sessionId}/interrupt`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    if (!response.ok) {
      throw new Error(`Failed to interrupt terminal: ${response.statusText}`);
    }
    
    return await response.json();
  } catch (error) {
    console.error('❌ Failed to interrupt terminal:', error);
    throw error;
  }
};

/**
 * Terminate session
 */
export const terminateSession = async (
  sessionId: string, 
  _reason?: string
): Promise<{ sessionId: string; reason: string; status: string }> => {
  try {
    const url = `${API_BASE_URL}/${sessionId}`;
    
    const response = await fetch(url, {
      method: 'DELETE',
    });
    
    if (!response.ok) {
      throw new Error(`Failed to terminate session: ${response.statusText}`);
    }
    
    return await response.json();
  } catch (error) {
    console.error('❌ Failed to terminate session:', error);
    throw error;
  }
};

/**
 * Get active session list
 */
export const listSessions = async (userId?: string): Promise<{ sessions: any[]; count: number }> => {
  try {
    let url = `${API_BASE_URL}`;
    
    // 如果提供了userId，只获取该用户的会话
    if (userId) {
      url = `${url}?userId=${encodeURIComponent(userId)}`;
    }
    
    const response = await fetch(url);
    
    if (!response.ok) {
      throw new Error(`Failed to list sessions: ${response.statusText}`);
    }
    
    const sessions = await response.json();
    
    return {
      sessions: sessions,
      count: sessions.length
    };
  } catch (error) {
    console.error('❌ Failed to list sessions:', error);
    throw error;
  }
};

/**
 * Execute command in session
 */
export const executeCommand = async (
  sessionId: string, 
  command: string,
  timeoutMs?: number
): Promise<string> => {
  try {
    const params = new URLSearchParams();
    params.append('command', command);
    if (timeoutMs) {
      params.append('timeoutMs', timeoutMs.toString());
    }
    
    const url = `${API_BASE_URL}/${sessionId}/execute?${params.toString()}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    if (!response.ok) {
      throw new Error(`Failed to execute command: ${response.statusText}`);
    }
    
    return await response.text();
  } catch (error) {
    console.error('❌ Failed to execute command:', error);
    throw error;
  }
};

/**
 * Execute command and check success
 */
export const executeCommandAndCheckSuccess = async (
  sessionId: string, 
  command: string
): Promise<boolean> => {
  try {
    const params = new URLSearchParams();
    params.append('command', command);
    
    const url = `${API_BASE_URL}/${sessionId}/execute-check?${params.toString()}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    if (!response.ok) {
      throw new Error(`Failed to execute command: ${response.statusText}`);
    }
    
    const result = await response.json();
    return result === true;
  } catch (error) {
    console.error('❌ Failed to execute command:', error);
    throw error;
  }
};

/**
 * Get session status
 */
export const getSessionStatus = async (sessionId: string): Promise<string> => {
  try {
    const url = `${API_BASE_URL}/${sessionId}/status`;
    
    const response = await fetch(url);
    
    if (!response.ok) {
      throw new Error(`Failed to get session status: ${response.statusText}`);
    }
    
    const result = await response.json();
    return result.status;
  } catch (error) {
    console.error('❌ Failed to get session status:', error);
    throw error;
  }
};

/**
 * Get session by ID
 */
export const getSessionById = async (sessionId: string): Promise<any> => {
  try {
    const url = `${API_BASE_URL}/${sessionId}`;
    
    const response = await fetch(url);
    
    if (!response.ok) {
      if (response.status === 404) {
        return null;
      }
      throw new Error(`Failed to get session: ${response.statusText}`);
    }
    
    return await response.json();
  } catch (error) {
    console.error('❌ Failed to get session:', error);
    throw error;
  }
};

/**
 * Check if session is active
 */
export const checkSessionActive = async (sessionId: string, _userId?: string): Promise<boolean> => {
  try {
    const status = await getSessionStatus(sessionId);
    return status === 'ACTIVE';
  } catch (error) {
    console.error('❌ Failed to check session status:', error);
    return false;
  }
};