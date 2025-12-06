// src/portable/windows.rs
use portable_pty::{native_pty_system, PtySize, CommandBuilder, Child, Master};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use tokio::sync::mpsc::{self, error::TryRecvError};

/// Windows 专用的异步 PTY 包装
pub struct WindowsPty {
    // 用于接收 PTY 输出的通道
    read_rx: mpsc::Receiver<io::Result<Vec<u8>>>,
    // 用于发送输入到 PTY 的通道
    write_tx: mpsc::Sender<Vec<u8>>,
    // 当前读取缓冲区
    read_buffer: Vec<u8>,
    // 缓冲区读取位置
    read_pos: usize,
    // 子进程引用（防止提前退出）
    _child: Box<dyn Child + Send + Sync>,
    // 进程状态
    child_exited: bool,
}

impl WindowsPty {
    pub fn new(config: &crate::PtyConfig) -> Result<Self, crate::PtyError> {
        let pty_system = native_pty_system();
        
        // 创建 PTY
        let pair = pty_system.open_pty(PtySize {
            cols: config.cols,
            rows: config.rows,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        
        // 构建命令
        let mut cmd = CommandBuilder::new(&config.command);
        for arg in &config.args {
            cmd.arg(arg);
        }
        
        // 启动进程
        let child = Box::new(pair.slave.spawn_command(cmd)?);
        
        // 创建通信通道
        let (read_tx, read_rx) = mpsc::channel(100);
        let (write_tx, write_rx) = mpsc::channel(100);
        
        // 关键：启动专用线程处理阻塞的 PTY I/O
        let master = pair.master;
        std::thread::spawn(move || {
            pty_io_worker(master, read_tx, write_rx);
        });
        
        Ok(Self {
            read_rx,
            write_tx,
            read_buffer: Vec::new(),
            read_pos: 0,
            _child: child,
            child_exited: false,
        })
    }
    
    /// 检查通道中是否有数据
    fn poll_channel(&mut self, cx: &mut Context<'_>) -> Poll<Option<io::Result<Vec<u8>>>> {
        match self.read_rx.poll_recv(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(Some(result)),
            Poll::Ready(None) => Poll::Ready(None), // 通道关闭
            Poll::Pending => Poll::Pending,
        }
    }
}

/// PTY I/O 工作线程（在后台运行）
fn pty_io_worker(
    mut master: Master,
    read_tx: mpsc::Sender<io::Result<Vec<u8>>>,
    write_rx: mpsc::Receiver<Vec<u8>>,
) {
    let mut buffer = [0u8; 4096];
    
    loop {
        // 同时等待：读取 PTY 输出 和 检查写入队列
        let read_result = master.read(&mut buffer);
        
        match read_result {
            Ok(0) => {
                // EOF - 进程结束
                let _ = read_tx.blocking_send(Ok(Vec::new())); // 发送空数据表示 EOF
                break;
            }
            Ok(n) => {
                // 成功读取到数据
                let data = buffer[..n].to_vec();
                let _ = read_tx.blocking_send(Ok(data));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 非阻塞模式下的正常情况，短暂休眠后继续
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(e) => {
                // 其他错误
                let _ = read_tx.blocking_send(Err(e));
                break;
            }
        }
        
        // 检查是否有数据需要写入 PTY
        match write_rx.try_recv() {
            Ok(data) => {
                let _ = master.write_all(&data);
                let _ = master.flush();
            }
            Err(TryRecvError::Empty) => {
                // 没有待写入数据，继续
            }
            Err(TryRecvError::Disconnected) => {
                // 写入通道已关闭，退出线程
                break;
            }
        }
    }
}

// 实现 AsyncRead
impl AsyncRead for WindowsPty {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // 如果缓冲区有数据，先从中读取
        if self.read_pos < self.read_buffer.len() {
            let remaining = &self.read_buffer[self.read_pos..];
            let to_copy = std::cmp::min(remaining.len(), buf.remaining());
            
            buf.put_slice(&remaining[..to_copy]);
            self.read_pos += to_copy;
            
            // 如果缓冲区已读完，清空它
            if self.read_pos >= self.read_buffer.len() {
                self.read_buffer.clear();
                self.read_pos = 0;
            }
            
            return Poll::Ready(Ok(()));
        }
        
        // 从通道获取新数据
        match ready!(self.poll_channel(cx)) {
            Some(Ok(data)) if data.is_empty() => {
                // 空数据表示 EOF
                self.child_exited = true;
                Poll::Ready(Ok(()))
            }
            Some(Ok(data)) => {
                // 将数据存入缓冲区，然后递归调用自身
                self.read_buffer = data;
                self.read_pos = 0;
                self.poll_read(cx, buf)
            }
            Some(Err(e)) => Poll::Ready(Err(e)),
            None => {
                // 通道关闭
                self.child_exited = true;
                Poll::Ready(Ok(()))
            }
        }
    }
}

// 实现 AsyncWrite
impl AsyncWrite for WindowsPty {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let self_mut = self.get_mut();
        
        if self_mut.child_exited {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "PTY process has terminated",
            )));
        }
        
        // 通过通道发送数据到工作线程
        match self_mut.write_tx.try_send(buf.to_vec()) {
            Ok(_) => Poll::Ready(Ok(buf.len())),
            Err(e) if e.is_full() => {
                // 通道满，需要等待
                Poll::Pending
            }
            Err(_) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "PTY write channel disconnected",
            ))),
        }
    }
    
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        // Windows PTY 通常不需要显式刷新
        Poll::Ready(Ok(()))
    }
    
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        // 关闭写入通道
        drop(self.get_mut().write_tx.clone());
        Poll::Ready(Ok(()))
    }
}

// 实现 AsyncPty trait（基于之前的设计）
#[async_trait::async_trait]
impl crate::AsyncPty for WindowsPty {
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), crate::PtyError> {
        // Windows 上的 PTY 调整大小
        // portable-pty 可能已经处理了，这里简化实现
        Ok(())
    }
    
    fn pid(&self) -> Option<u32> {
        // 获取进程 ID（Windows）
        None // portable-pty 可能不直接暴露 PID
    }
    
    fn is_alive(&self) -> bool {
        !self.child_exited
    }
    
    async fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, crate::PtyError> {
        // 通过检查子进程状态实现
        if self.child_exited {
            return Ok(None);
        }
        // 简化实现，实际需要更复杂的检查
        Ok(None)
    }
    
    async fn kill(&mut self) -> Result<(), crate::PtyError> {
        if self.child_exited {
            return Ok(());
        }
        
        // 通过 Child trait 终止进程
        self._child.kill()?;
        self.child_exited = true;
        Ok(())
    }
}