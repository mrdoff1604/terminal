use crate::pty::pty_trait::{AsyncPty, PtyConfig, PtyError, PtyFactory};
use async_trait::async_trait;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_pty_process::{Command, PtyMaster};
use tracing::{debug, error, info};

/// 基于 tokio-pty-process 的 PTY 实现
/// 提供真正的 PTY 支持，适用于 Unix-like 系统
pub struct TokioPtyProcessPty {
    child: tokio::process::Child,
    master: PtyMaster,
    child_exited: bool,
    cols: u16,
    rows: u16,
}

impl TokioPtyProcessPty {
    pub fn new(config: &PtyConfig) -> Result<Self, PtyError> {
        info!(
            "TokioPtyProcessPty: Creating PTY with command: {:?}, args: {:?}",
            config.command, config.args
        );

        // 构建命令 - 使用 tokio-pty-process::Command
        let mut cmd = Command::new(&config.command);

        // 添加配置文件中指定的参数
        for arg in &config.args {
            cmd.arg(arg);
        }

        // 设置工作目录
        if let Some(cwd) = &config.cwd {
            cmd.current_dir(cwd);
            info!("TokioPtyProcessPty: Setting cwd to: {:?}", cwd);
        }

        // 设置环境变量 - 完全遵循配置文件，不添加任何硬编码参数
        for (key, value) in &config.env {
            cmd.env(key, value);
            if key == "PATH" || key == "TERM" {
                info!("TokioPtyProcessPty: Setting env {}={:?}", key, value);
            }
        }

        // 设置 PTY 大小
        cmd.size(config.cols, config.rows);

        // 使用 tokio-pty-process 生成带 PTY 的子进程
        let pty = cmd.spawn().map_err(|e| {
            error!("TokioPtyProcessPty: Failed to spawn PTY process: {}", e);
            PtyError::SpawnFailed(e.to_string())
        })?;

        info!("TokioPtyProcessPty: Successfully created PTY process");

        Ok(Self {
            child: pty.child,
            master: pty.master,
            child_exited: false,
            cols: config.cols,
            rows: config.rows,
        })
    }
}

// 实现 AsyncRead for TokioPtyProcessPty
impl AsyncRead for TokioPtyProcessPty {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let self_mut = self.get_mut();

        // 检查进程是否已退出
        if let Ok(Some(status)) = self_mut.child.try_wait() {
            debug!(
                "TokioPtyProcessPty: Child process exited with status: {:?}",
                status
            );
            self_mut.child_exited = true;
        }

        // 从 PTY master 读取数据
        Pin::new(&mut self_mut.master).poll_read(cx, buf)
    }
}

// 实现 AsyncWrite for TokioPtyProcessPty
impl AsyncWrite for TokioPtyProcessPty {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let self_mut = self.get_mut();

        // 检查进程是否已退出
        if self_mut.child_exited {
            return Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "PTY process has exited",
            )));
        }

        // 向 PTY master 写入数据
        Pin::new(&mut self_mut.master).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let self_mut = self.get_mut();
        
        // 刷新 PTY master 写入缓冲区
        Pin::new(&mut self_mut.master).poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let self_mut = self.get_mut();
        
        // 关闭 PTY master 写入端
        Pin::new(&mut self_mut.master).poll_shutdown(cx)
    }
}

// 实现 AsyncPty trait 为 TokioPtyProcessPty
#[async_trait]
impl AsyncPty for TokioPtyProcessPty {
    /// 调整终端大小
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        info!(
            "TokioPtyProcessPty: Resizing PTY from {}x{} to {}x{}",
            self.cols, self.rows, cols, rows
        );

        // 调整 PTY 大小
        self.master.resize(cols, rows).map_err(|e| {
            error!("TokioPtyProcessPty: Failed to resize PTY: {}", e);
            PtyError::ResizeFailed(e.to_string())
        })?;

        // 更新本地记录的大小
        self.cols = cols;
        self.rows = rows;
        info!("TokioPtyProcessPty: PTY resized successfully");
        Ok(())
    }

    /// 获取进程ID（如果可用）
    fn pid(&self) -> Option<u32> {
        self.child.id()
    }

    /// 检查进程是否存活
    fn is_alive(&self) -> bool {
        !self.child_exited
    }

    /// 等待进程结束（非阻塞检查）
    async fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, PtyError> {
        // 尝试等待进程结束（非阻塞）
        if self.child_exited {
            return Ok(None);
        }

        match self.child.try_wait() {
            Ok(Some(status)) => {
                info!(
                    "TokioPtyProcessPty: Child process exited with status: {:?}",
                    status
                );
                self.child_exited = true;
                Ok(Some(status))
            }
            Ok(None) => {
                debug!("TokioPtyProcessPty: Child process still running");
                Ok(None)
            }
            Err(e) => {
                error!("TokioPtyProcessPty: Failed to check child status: {}", e);
                Err(PtyError::Other(e.to_string()))
            }
        }
    }

    /// 立即终止进程
    async fn kill(&mut self) -> Result<(), PtyError> {
        info!("TokioPtyProcessPty: Killing child process");

        self.child.kill().await.map_err(|e| {
            error!("TokioPtyProcessPty: Failed to kill child process: {}", e);
            PtyError::Other(e.to_string())
        })?;

        self.child_exited = true;
        Ok(())
    }
}

// ================ 工厂实现 ================

/// 基于 tokio-pty-process 的 PTY 工厂
pub struct TokioPtyProcessPtyFactory;

#[async_trait]
impl PtyFactory for TokioPtyProcessPtyFactory {
    async fn create(&self, config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        let pty = TokioPtyProcessPty::new(config)?;
        Ok(Box::new(pty))
    }

    fn name(&self) -> &'static str {
        "tokio-pty-process"
    }
}
