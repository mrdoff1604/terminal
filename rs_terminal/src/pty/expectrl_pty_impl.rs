use std::process::ExitStatus;
use std::sync::Mutex;

use async_trait::async_trait;
use expectrl::spawn;

use crate::pty::{AsyncPty, PtyConfig, PtyError, PtyFactory};

/// 基于expectrl库的PTY实现
pub struct ExpectrlPty {
    #[cfg(unix)]
    // 使用动态类型避免泛型参数问题
    session: Mutex<Box<dyn std::any::Any + Send + Sync>>,
    pid: Option<u32>,
    child_exited: bool,
}

#[async_trait]
impl AsyncPty for ExpectrlPty {
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        #[cfg(unix)]
        {
            Err(PtyError::Other("resize not implemented for expectrl-pty".to_string()))
        }
        
        #[cfg(not(unix))]
        {
            Err(PtyError::NotAvailable)
        }
    }

    fn pid(&self) -> Option<u32> {
        self.pid
    }

    fn is_alive(&self) -> bool {
        !self.child_exited
    }

    async fn try_wait(&mut self) -> Result<Option<ExitStatus>, PtyError> {
        #[cfg(unix)]
        {
            Err(PtyError::Other("try_wait not implemented for expectrl-pty".to_string()))
        }
        
        #[cfg(not(unix))]
        {
            Err(PtyError::NotAvailable)
        }
    }

    async fn kill(&mut self) -> Result<(), PtyError> {
        #[cfg(unix)]
        {
            Err(PtyError::Other("kill not implemented for expectrl-pty".to_string()))
        }
        
        #[cfg(not(unix))]
        {
            Err(PtyError::NotAvailable)
        }
    }
}

// 实现AsyncRead，直接使用Session的异步读取功能
impl tokio::io::AsyncRead for ExpectrlPty {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        #[cfg(unix)]
        {
            // 注意：expectrl库的Session类型可能没有直接实现AsyncRead
            // 这里我们返回一个错误，表示尚未实现
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AsyncRead not implemented for expectrl-pty"
            )))
        }
        
        #[cfg(not(unix))]
        {
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Not available on non-Unix platforms"
            )))
        }
    }
}

// 实现AsyncWrite，直接使用Session的异步写入功能
impl tokio::io::AsyncWrite for ExpectrlPty {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        #[cfg(unix)]
        {
            // 注意：expectrl库的Session类型可能没有直接实现AsyncWrite
            // 这里我们返回一个错误，表示尚未实现
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AsyncWrite not implemented for expectrl-pty"
            )))
        }
        
        #[cfg(not(unix))]
        {
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Not available on non-Unix platforms"
            )))
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        #[cfg(unix)]
        {
            // 注意：expectrl库的Session类型可能没有直接实现AsyncWrite
            // 这里我们返回一个错误，表示尚未实现
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AsyncWrite not implemented for expectrl-pty"
            )))
        }
        
        #[cfg(not(unix))]
        {
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Not available on non-Unix platforms"
            )))
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        #[cfg(unix)]
        {
            // 注意：expectrl库的Session类型可能没有直接实现AsyncWrite
            // 这里我们返回一个错误，表示尚未实现
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AsyncWrite not implemented for expectrl-pty"
            )))
        }
        
        #[cfg(not(unix))]
        {
            std::task::Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Not available on non-Unix platforms"
            )))
        }
    }
}

/// Expectrl PTY工厂
pub struct ExpectrlPtyFactory;

#[async_trait]
impl PtyFactory for ExpectrlPtyFactory {
    async fn create(&self, config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        #[cfg(unix)]
        {
            // 构建命令行字符串
            let cmd_str = format!("{}", config.command);
            
            // 生成并启动会话
            let session = spawn(cmd_str).map_err(|e| {
                PtyError::SpawnFailed(e.to_string())
            })?;
            
            // 使用动态类型来存储会话
            let session_box: Box<dyn std::any::Any + Send + Sync> = Box::new(session);
            
            // PID初始化为None，因为expectrl库可能不直接提供PID访问
            let pid = None;
            
            Ok(Box::new(ExpectrlPty {
                session: Mutex::new(session_box),
                pid,
                child_exited: false,
            }))
        }
        
        #[cfg(not(unix))]
        {
            Err(PtyError::NotAvailable)
        }
    }

    fn name(&self) -> &'static str {
        "expectrl-pty"
    }
}
