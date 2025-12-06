use crate::pty::pty_trait::{PtyConfig, PtyError, AsyncPty, PtyFactory};
use async_trait::async_trait;
use portable_pty::{native_pty_system, PtySize, CommandBuilder, Child, ExitStatus};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::io::unix::AsyncFd;

/// 基于portable-pty的Unix PTY实现
pub struct UnixPty {
    inner: AsyncFd<portable_pty::PtyMaster>,
    child: Box<dyn Child + Send + Sync>,
    child_exited: bool,
}

impl UnixPty {
    pub fn new(config: &PtyConfig) -> Result<Self, PtyError> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            cols: config.cols,
            rows: config.rows,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // 构建命令
        let mut cmd_builder = CommandBuilder::new(&config.command);
        
        // 添加参数
        for arg in &config.args {
            cmd_builder.arg(arg);
        }
        
        // 设置环境变量
        for (key, value) in &config.env {
            cmd_builder.env(key, value);
        }
        
        // 设置工作目录
        if let Some(cwd) = &config.cwd {
            cmd_builder.cwd(cwd);
        }

        // 启动子进程
        let child = Box::new(pair.slave.spawn_command(cmd_builder)?);

        // 用AsyncFd包装
        let async_fd = AsyncFd::new(pair.master)?;

        Ok(Self {
            inner: async_fd,
            child,
            child_exited: false,
        })
    }
}

// 实现AsyncRead
impl AsyncRead for UnixPty {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        
        // 如果子进程已退出且无数据可读，返回EOF
        if self_mut.child_exited {
            return Poll::Ready(Ok(()));
        }

        loop {
            // 等待文件描述符可读
            let mut guard = match self_mut.inner.poll_read_ready(cx) {
                Poll::Ready(Ok(guard)) => guard,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            };

            // 尝试读取
            match guard.try_io(|inner| {
                let fd = inner.get_ref();
                let dst = buf.initialize_unfilled();
                
                match fd.read(dst) {
                    Ok(0) => {
                        // EOF - 子进程可能已关闭输出
                        Ok(0)
                    }
                    Ok(n) => {
                        buf.advance(n);
                        Ok(n)
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // 数据未就绪，需要重新等待
                        Err(io::Error::new(io::ErrorKind::WouldBlock, "retry"))
                    }
                    Err(e) => Err(e),
                }
            }) {
                Ok(0) => {
                    // 遇到EOF，标记子进程可能已退出
                    self_mut.child_exited = true;
                    return Poll::Ready(Ok(()));
                }
                Ok(_) => return Poll::Ready(Ok(())),
                Err(_would_block) => {
                    // 通知可能已过时，继续循环
                    continue;
                }
            }
        }
    }
}

// 实现AsyncWrite
impl AsyncWrite for UnixPty {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let self_mut = self.get_mut();
        
        if self_mut.child_exited {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "PTY process has terminated",
            )));
        }

        loop {
            let mut guard = match self_mut.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(guard)) => guard,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            };

            match guard.try_io(|inner| inner.get_ref().write(buf)) {
                Ok(result) => return Poll::Ready(result),
                Err(_) => continue, // 遇到WouldBlock，重试
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[async_trait]
impl AsyncPty for UnixPty {
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        if self.child_exited {
            return Err(PtyError::ProcessTerminated);
        }
        
        // 获取PTY master并调整大小
        let pty_master = self.inner.get_ref();
        pty_master.resize(PtySize {
            cols,
            rows,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        
        Ok(())
    }
    
    fn pid(&self) -> Option<u32> {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            // portable-pty 可能不直接暴露PID
            None
        }
        #[cfg(not(unix))]
        None
    }
    
    fn is_alive(&self) -> bool {
        !self.child_exited
    }
    
    async fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, PtyError> {
        if self.child_exited {
            return Ok(None);
        }
        
        // 检查子进程状态（非阻塞）
        if let Some(status) = self.child.try_wait()? {
            self.child_exited = true;
            
            // Convert portable_pty::ExitStatus to std::process::ExitStatus
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                Ok(Some(std::process::ExitStatus::from_raw(status.code() as i32)))
            }
            #[cfg(windows)]
            {
                Ok(Some(std::process::ExitStatus::from_raw(status.code() as u32)))
            }
        } else {
            Ok(None)
        }
    }
    
    async fn kill(&mut self) -> Result<(), PtyError> {
        if self.child_exited {
            return Ok(());
        }
        
        self.child.kill()?;
        self.child_exited = true;
        Ok(())
    }
}

// ================ 工厂实现 ================

pub struct UnixPtyFactory;

#[async_trait]
impl PtyFactory for UnixPtyFactory {
    async fn create(&self, config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        let pty = UnixPty::new(config)?;
        Ok(Box::new(pty))
    }
    
    fn name(&self) -> &'static str {
        "unix-pty"
    }
}