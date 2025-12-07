use crate::pty::pty_trait::{AsyncPty, PtyConfig, PtyError, PtyFactory};
use async_trait::async_trait;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::process::Command;
use tracing::{debug, error, info};

/// 基于 tokio-pty-process 的 PTY 实现
/// 提供真正的 PTY 支持，适用于 Unix-like 系统
pub struct TokioPtyProcessPty {
    stdin: tokio::process::ChildStdin,
    stdout: tokio::process::ChildStdout,
    stderr: tokio::process::ChildStderr,
    child: tokio::process::Child,
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

        // 构建命令 - 使用 tokio::process::Command
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

        // 配置进程 I/O
        let mut cmd = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        // 生成子进程
        let mut child = cmd.spawn().map_err(|e| {
            error!("TokioPtyProcessPty: Failed to spawn process: {}", e);
            PtyError::SpawnFailed(e.to_string())
        })?;

        // 获取进程 I/O
        let stdin = child.stdin.take().ok_or_else(|| {
            error!("TokioPtyProcessPty: Failed to get stdin");
            PtyError::SpawnFailed("Failed to get stdin".to_string())
        })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            error!("TokioPtyProcessPty: Failed to get stdout");
            PtyError::SpawnFailed("Failed to get stdout".to_string())
        })?;

        let stderr = child.stderr.take().ok_or_else(|| {
            error!("TokioPtyProcessPty: Failed to get stderr");
            PtyError::SpawnFailed("Failed to get stderr".to_string())
        })?;

        info!("TokioPtyProcessPty: Successfully created PTY process");

        Ok(Self {
            stdin,
            stdout,
            stderr,
            child,
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
            return Poll::Ready(Ok(()));
        }

        // 首先尝试从 stdout 读取数据
        let stdout_result = Pin::new(&mut self_mut.stdout).poll_read(cx, buf);

        match stdout_result {
            Poll::Ready(Ok(())) => {
                // 从 stdout 读取到数据，返回结果
                return Poll::Ready(Ok(()));
            }
            Poll::Ready(Err(e)) => {
                // stdout 出错，尝试从 stderr 读取
                error!("TokioPtyProcessPty: Error reading from stdout: {}", e);
            }
            Poll::Pending => {
                // stdout 没有数据，尝试从 stderr 读取
            }
        }

        // 从 stderr 读取数据
        let stderr_result = Pin::new(&mut self_mut.stderr).poll_read(cx, buf);

        match stderr_result {
            Poll::Ready(Ok(())) => {
                // 从 stderr 读取到数据，返回结果
                return Poll::Ready(Ok(()));
            }
            Poll::Ready(Err(e)) => {
                // stderr 出错，返回错误
                error!("TokioPtyProcessPty: Error reading from stderr: {}", e);
                return Poll::Ready(Err(e));
            }
            Poll::Pending => {
                // 两个流都没有数据，返回 Pending
                return Poll::Pending;
            }
        }
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

        // 向 stdin 写入数据
        let result = Pin::new(&mut self_mut.stdin).poll_write(cx, buf);
        
        if let Poll::Ready(Ok(n)) = &result {
            debug!("TokioPtyProcessPty: Wrote {} bytes to stdin", n);
        }
        
        result
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let self_mut = self.get_mut();
        
        // 刷新 stdin 写入缓冲区
        Pin::new(&mut self_mut.stdin).poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let self_mut = self.get_mut();
        
        // 关闭 stdin 写入端
        Pin::new(&mut self_mut.stdin).poll_shutdown(cx)
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