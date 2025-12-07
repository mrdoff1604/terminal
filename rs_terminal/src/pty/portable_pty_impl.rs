use crate::pty::pty_trait::{AsyncPty, PtyConfig, PtyError, PtyFactory};
use async_trait::async_trait;
use portable_pty::{Child, CommandBuilder, PtySize};
use std::pin::Pin;
use std::process::ExitStatus as StdExitStatus;
use std::sync::Mutex;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tracing::info;

/// 基于 portable-pty 库的异步 PTY 实现
pub struct PortablePty {
    cols: u16,
    rows: u16,
    master: Mutex<Box<dyn portable_pty::MasterPty + Send>>,
    reader: Mutex<Box<dyn std::io::Read + Send>>,
    writer: Mutex<Box<dyn std::io::Write + Send>>,
    child: Mutex<Box<dyn Child + Send>>,
    child_exited: bool,
}

impl PortablePty {
    pub fn new(config: &PtyConfig) -> Result<Self, PtyError> {
        info!(
            "PortablePty: Creating PTY with command: {:?}, args: {:?}",
            config.command, config.args
        );

        // Get the default PTY system
        let pty_system = portable_pty::native_pty_system();

        // Create PTY pair
        let pair = pty_system.openpty(PtySize {
            rows: config.rows,
            cols: config.cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Create command builder
        let mut cmd = CommandBuilder::new(config.command.clone());
        cmd.args(&config.args);

        // Set environment variables
        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        // Set working directory if provided
        if let Some(cwd) = &config.cwd {
            cmd.cwd(cwd);
        }

        // Spawn the child process
        let child = pair.slave.spawn_command(cmd)?;
        
        // Create reader and writer
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        Ok(Self {
            cols: config.cols,
            rows: config.rows,
            master: Mutex::new(pair.master),
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
            child: Mutex::new(child),
            child_exited: false,
        })
    }
}

// 实现 AsyncRead for PortablePty
impl AsyncRead for PortablePty {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // 保存必要的信息以便在 future 中使用
        let this_ptr = self.get_mut() as *mut PortablePty;
        let buf_len = buf.initialize_unfilled().len();
        let mut local_buf = vec![0; buf_len];
        
        // 创建一个 future 来处理异步读取
        // 注意：这里我们使用一个简单的实现，实际项目中应该使用更高效的方法
        let result = unsafe {
            let this_ref = &mut *this_ptr;
            let mut reader = this_ref.reader.lock().unwrap();
            
            reader.read(&mut local_buf)
        };
        
        match result {
            Ok(n) => {
                buf.put_slice(&local_buf[..n]);
                Poll::Ready(Ok(()))
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                Poll::Pending
            }
            Err(e) => {
                Poll::Ready(Err(e))
            }
        }
    }
}

// 实现 AsyncWrite for PortablePty
impl AsyncWrite for PortablePty {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let this = self.get_mut();
        let mut writer = this.writer.lock().unwrap();
        
        // 直接使用同步写入，因为这是在 poll_write 方法中
        // 注意：这不是最优实现，实际项目中应该使用 tokio::task::spawn_blocking
        match writer.write(buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        let this = self.get_mut();
        let mut writer = this.writer.lock().unwrap();
        
        // 直接使用同步刷新
        match writer.flush() {
            Ok(()) => Poll::Ready(Ok(())),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        // PTY 不需要特殊的关闭处理
        Poll::Ready(Ok(()))
    }
}

// 实现 AsyncPty trait 为 PortablePty
#[async_trait]
impl AsyncPty for PortablePty {
    /// 调整终端大小
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        info!("PortablePty: Resizing PTY to {}x{}", cols, rows);

        let master = self.master.lock().unwrap();
        master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        self.cols = cols;
        self.rows = rows;
        Ok(())
    }

    /// 获取进程ID（如果可用）
    fn pid(&self) -> Option<u32> {
        // portable-pty 的 Child 没有 id() 方法，返回 None
        None
    }

    /// 检查进程是否存活
    fn is_alive(&self) -> bool {
        !self.child_exited
    }

    /// 等待进程结束（非阻塞检查）
    async fn try_wait(&mut self) -> Result<Option<StdExitStatus>, PtyError> {
        let mut child = self.child.lock().unwrap();

        if self.child_exited {
            return Ok(None);
        }

        match child.try_wait()? {
            Some(_status) => {
                self.child_exited = true;
                // portable-pty 的 ExitStatus 与 std::process::ExitStatus 不同，返回一个简单的成功状态
                Ok(Some(StdExitStatus::default()))
            }
            None => Ok(None),
        }
    }

    /// 立即终止进程
    async fn kill(&mut self) -> Result<(), PtyError> {
        info!("PortablePty: Killing child process");

        let mut child = self.child.lock().unwrap();
        child.kill()?;
        self.child_exited = true;
        Ok(())
    }
}

// ================ 工厂实现 ================

/// 基于 portable-pty 的 PTY 工厂
pub struct PortablePtyFactory;

#[async_trait]
impl PtyFactory for PortablePtyFactory {
    async fn create(&self, config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        let pty = PortablePty::new(config)?;
        Ok(Box::new(pty))
    }

    fn name(&self) -> &'static str {
        "portable-pty"
    }
}
