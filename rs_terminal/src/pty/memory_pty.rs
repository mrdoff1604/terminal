/// Pure async memory-based PTY implementation for testing and development
use async_trait::async_trait;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::Mutex;

use crate::pty::pty_trait::{AsyncPty, PtyConfig, PtyError, PtyFactory};

/// Pure async memory-based PTY implementation
pub struct MemoryPty {
    // Use a buffer to simulate PTY output with Arc for sharing
    output_buffer: Arc<Mutex<Vec<u8>>>,
    // Flag to indicate if PTY is alive with Arc for sharing
    alive: Arc<Mutex<bool>>,
    // Command being executed
    command: String,
    // Current directory
    cwd: Option<String>,
}

impl MemoryPty {
    /// Create a new MemoryPty instance
    pub async fn new(config: &PtyConfig) -> Result<Self, PtyError> {
        // Initialize with a welcome message
        let output_buffer = Arc::new(Mutex::new(format!("Memory PTY initialized for command: {}\r\n", config.command).into_bytes()));
        
        // Convert PathBuf to String if needed
        let cwd_str = config.cwd.as_ref().and_then(|path| path.to_str().map(|s| s.to_string()));
        
        Ok(Self {
            output_buffer,
            alive: Arc::new(Mutex::new(true)),
            command: config.command.clone(),
            cwd: cwd_str,
        })
    }
    
    /// Simulate processing input commands and generating output
    async fn process_input(&self, input: &[u8]) -> Vec<u8> {
        let input_str = match String::from_utf8_lossy(input) {
            s => s.to_string(),
        };
        
        // Simulate command processing
        let mut response = String::new();
        
        for line in input_str.lines() {
            let trimmed = line.trim();
            match trimmed {
                "ls" | "dir" => {
                    response.push_str("Memory PTY Directory Contents:\r\n");
                    response.push_str("  README.md\r\n");
                    response.push_str("  Cargo.toml\r\n");
                    response.push_str("  src/\r\n");
                },
                "pwd" | "cd " => {
                    if let Some(cwd) = &self.cwd {
                        response.push_str(&format!("{}\r\n", cwd));
                    } else {
                        response.push_str("/\r\n");
                    }
                },
                "echo" => {
                    response.push_str(&format!("{}\r\n", line.trim_start_matches("echo").trim()));
                },
                "exit" => {
                    let mut alive = self.alive.lock().await;
                    *alive = false;
                    response.push_str("Exiting Memory PTY...\r\n");
                },
                "help" => {
                    response.push_str("Memory PTY Commands:\r\n");
                    response.push_str("  ls/dir - List directory contents\r\n");
                    response.push_str("  pwd/cd - Show current directory\r\n");
                    response.push_str("  echo - Echo text\r\n");
                    response.push_str("  exit - Exit PTY\r\n");
                    response.push_str("  help - Show this help\r\n");
                },
                _ if !trimmed.is_empty() => {
                    response.push_str(&format!("Command not found: {}\r\n", trimmed));
                },
                _ => {},
            }
        }
        
        response.into_bytes()
    }
}

// Implement AsyncRead for MemoryPty
impl AsyncRead for MemoryPty {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = self.get_mut();
        
        // Check if we have data in the output buffer
        let mut output_buf = this.output_buffer.try_lock().unwrap();
        
        if !output_buf.is_empty() {
            // Copy as much data as possible to the buffer
            let len = std::cmp::min(output_buf.len(), buf.remaining());
            buf.put_slice(&output_buf[..len]);
            output_buf.drain(..len);
            
            std::task::Poll::Ready(Ok(()))
        } else {
            // No data available, return Pending
            std::task::Poll::Pending
        }
    }
}

// Implement AsyncWrite for MemoryPty
impl AsyncWrite for MemoryPty {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        data: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        let this = self.get_mut();
        
        // Check if PTY is alive
        if !*this.alive.try_lock().unwrap() {
            return std::task::Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "PTY is not alive",
            )));
        }
        
        // Simulate processing input asynchronously
        let this_clone = this.clone();
        let data_clone = data.to_vec();
        
        // Process the input in a separate task to keep this non-blocking
        tokio::spawn(async move {
            let response = this_clone.process_input(&data_clone).await;
            let mut output_buf = this_clone.output_buffer.lock().await;
            output_buf.extend_from_slice(&response);
        });
        
        // Return immediately with the full write length
        std::task::Poll::Ready(Ok(data.len()))
    }
    
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        // Memory PTY is always flushed
        std::task::Poll::Ready(Ok(()))
    }
    
    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        // Memory PTY shutdown is immediate
        std::task::Poll::Ready(Ok(()))
    }
}

#[async_trait]
impl AsyncPty for MemoryPty {
    async fn resize(&mut self, _cols: u16, _rows: u16) -> Result<(), PtyError> {
        // Memory PTY doesn't need resizing
        Ok(())
    }
    
    fn pid(&self) -> Option<u32> {
        // Memory PTY doesn't have a real PID
        None
    }
    
    fn is_alive(&self) -> bool {
        *self.alive.try_lock().unwrap()
    }
    
    async fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, PtyError> {
        if !*self.alive.lock().await {
            Ok(Some(std::process::ExitStatus::default()))
        } else {
            Ok(None)
        }
    }
    
    async fn kill(&mut self) -> Result<(), PtyError> {
        let mut alive = self.alive.lock().await;
        *alive = false;
        Ok(())
    }
}

// Implement Clone for MemoryPty to allow sharing between tasks
impl Clone for MemoryPty {
    fn clone(&self) -> Self {
        Self {
            output_buffer: self.output_buffer.clone(),
            alive: self.alive.clone(),
            command: self.command.clone(),
            cwd: self.cwd.clone(),
        }
    }
}

/// Memory PTY factory for creating MemoryPty instances
#[derive(Default)]
pub struct MemoryPtyFactory;

#[async_trait]
impl PtyFactory for MemoryPtyFactory {
    async fn create(&self, config: &PtyConfig) -> Result<Box<dyn AsyncPty>, PtyError> {
        let pty = MemoryPty::new(config).await?;
        Ok(Box::new(pty))
    }
    
    fn name(&self) -> &'static str {
        "memory-pty"
    }
}
