use std::process::ExitStatus;
use std::sync::Mutex;

use async_trait::async_trait;
use expectrl::{spawn, session::Session};

use crate::pty::{AsyncPty, PtyConfig, PtyError, PtyFactory};

/// 基于expectrl库的PTY实现
#[cfg(unix)]
pub struct ExpectrlPty {
    // 直接使用Session类型，指定具体的泛型参数
    session: Mutex<Session<std::process::Child, expectrl::stream::tokio::Stream>>,
    pid: Option<u32>,
    child_exited: bool,
}

#[cfg(not(unix))]
pub struct ExpectrlPty {
    pid: Option<u32>,
    child_exited: bool,
}

#[async_trait]
impl AsyncPty for ExpectrlPty {
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        #[cfg(unix)]
        {
            let mut session = self.session.lock().unwrap();
            session.resize(cols, rows).await.map_err(|e| {
                PtyError::ResizeFailed(e.to_string())
            })
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
            let mut session = self.session.lock().unwrap();
            
            match session.try_status() {
                Ok(Some(status)) => {
                    self.child_exited = true;
                    Ok(Some(status))
                },
                Ok(None) => {
                    Ok(None)
                },
                Err(e) => {
                    Err(PtyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
                },
            }
        }
        
        #[cfg(not(unix))]
        {
            Err(PtyError::NotAvailable)
        }
    }

    async fn kill(&mut self) -> Result<(), PtyError> {
        #[cfg(unix)]
        {
            let mut session = self.session.lock().unwrap();
            session.kill().await.map_err(|e| {
                PtyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
            })
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
            let this = self.get_mut();
            let mut session = this.session.lock().unwrap();
            
            // 直接委托给Session的AsyncRead实现
            tokio::io::AsyncRead::poll_read(
                std::pin::Pin::new(&mut *session),
                cx,
                buf
            )
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
            let this = self.get_mut();
            let mut session = this.session.lock().unwrap();
            
            // 直接委托给Session的AsyncWrite实现
            tokio::io::AsyncWrite::poll_write(
                std::pin::Pin::new(&mut *session),
                cx,
                buf
            )
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
            let this = self.get_mut();
            let mut session = this.session.lock().unwrap();
            
            // 直接委托给Session的AsyncWrite实现
            tokio::io::AsyncWrite::poll_flush(
                std::pin::Pin::new(&mut *session),
                cx
            )
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
            let this = self.get_mut();
            let mut session = this.session.lock().unwrap();
            
            // 直接委托给Session的AsyncWrite实现
            tokio::io::AsyncWrite::poll_shutdown(
                std::pin::Pin::new(&mut *session),
                cx
            )
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
            
            // 尝试获取PID
            // 注意：expectrl库的Session可能没有直接的pid方法
            let pid = None;
            
            Ok(Box::new(ExpectrlPty {
                session: Mutex::new(session),
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
