// Application Configuration
// Centralized configuration for all environment-specific settings

// 动态获取当前环境的基础URL
const getBaseUrl = () => {
  // 开发环境使用环境变量，生产环境使用相对路径
  if (import.meta.env.DEV) {
    return 'http://localhost:8080';
  } else {
    // 生产环境使用当前域名，支持前端打包后放在后端static目录下
    return window.location.origin;
  }
};

// 动态获取WebSocket URL
const getWsUrl = () => {
  if (import.meta.env.DEV) {
    return 'ws://localhost:8080';
  } else {
    // 生产环境根据当前协议生成正确的WebSocket URL
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    return `${protocol}//${window.location.host}`;
  }
};

// 动态获取WebTransport URL
const getWebTransportUrl = () => {
  if (import.meta.env.DEV) {
    return 'https://localhost:8082';
  } else {
    // WebTransport使用https协议
    return window.location.origin;
  }
};

export const APP_CONFIG = {
  // WebSocket server configuration
  WS_SERVER: {
    URL: getWsUrl(),
    PATH: '/ws'
  },
  
  // WebTransport server configuration
  WEBTRANSPORT_SERVER: {
    URL: getWebTransportUrl(),
    PATH: '/webtransport'
  },
  
  // API server configuration
  API_SERVER: {
    URL: getBaseUrl(),
    BASE_PATH: '/api'
  },
  
  // Application settings
  APP: {
    NAME: 'Waylon Terminal',
    VERSION: 'v1.0.0',
    DESCRIPTION: 'Enterprise Web Terminal'
  }
} as const;

export default APP_CONFIG;