// src/portable/windows.rs
use super::pty_trait::{AsyncPty, PtyConfig, PtyError, PtyFactory};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Windows 专用的异步 PTY 包装
pub struct WindowsPty {
    // 简化实现，返回NotAvailable
}

impl WindowsPty {
    pub fn new(_config: &PtyConfig) -> Result<Self, PtyError> {
        Err(PtyError::NotAvailable)
    }
}

// 实现 AsyncRead
impl AsyncRead for WindowsPty {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Poll::Ready(Err(io::Error::new(
            io::ErrorKind::Other,
            "PTY not available",
        )))
    }
}

// 实现 AsyncWrite
impl AsyncWrite for WindowsPty {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Err(io::Error::new(
            io::ErrorKind::Other,
            "PTY not available",
        )))
    }
    
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

// 实现 AsyncPty trait
#[async_trait::async_trait]
impl AsyncPty for WindowsPty {
    async fn resize(&mut self, _cols: u16, _rows: u16) -> Result<(), PtyError> {
        Err(PtyError::NotAvailable)
    }
    
    fn pid(&self) -> Option<u32> {
        None
    }
    
    fn is_alive(&self) -> bool {
        false
    }
    
    async fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, PtyError> {
        Err(PtyError::NotAvailable)
    }
    
    async fn kill(&mut self) -> Result<(), PtyError> {
        Ok(())
    }
}

// ================ 工厂实现 ================

/// Windows PTY factory for creating WindowsPty instances
pub struct WindowsPtyFactory;

impl Default for WindowsPtyFactory {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl PtyFactory for WindowsPtyFactory {
    async fn create(&self, _config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        // 返回NotAvailable错误，简化实现
        Err(PtyError::NotAvailable)
    }
    
    fn name(&self) -> &'static str {
        "windows-pty"
    }
}