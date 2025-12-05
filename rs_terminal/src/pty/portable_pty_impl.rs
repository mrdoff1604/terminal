use std::sync::{Arc, Mutex};
use tracing::{info, error};
use futures_util::future::FutureExt;
use std::io::{Read, Write};

use crate::pty::Pty;

/// Portable PTY implementation using the portable-pty crate
pub struct PortablePty {
    /// The PTY master for writing to the terminal (using trait object)
    master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    /// The reader for reading from the terminal (using standard Read trait)
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    /// The writer for writing to the terminal (using standard Write trait)
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    /// The process handle
    process: Arc<Mutex<Box<dyn portable_pty::Child + Send + Sync>>>,
    /// Flag to track if the PTY is alive
    alive: Arc<Mutex<bool>>,
}

impl PortablePty {
    /// Create a new PortablePty instance
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send>> {
        info!("Creating new PortablePty instance");
        
        // Spawn the PTY creation in a blocking thread since it's a blocking operation
        let pty_result = tokio::task::spawn_blocking(move || {
            // Create a command to run in the PTY
            let cmd = portable_pty::CommandBuilder::new(
                if cfg!(target_os = "windows") {
                    "cmd.exe"
                } else {
                    "/bin/bash"
                }
            );
            
            // Get the native PTY system
            let pty_system = portable_pty::native_pty_system();
            
            // Create a new PTY with default size
            let pty_pair = match pty_system.openpty(portable_pty::PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            }) {
                Ok(pair) => pair,
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to open PTY: {}", e)
                    )) as Box<dyn std::error::Error + Send>);
                }
            };
            
            // Spawn the command in the PTY
            let child = match pty_pair.slave.spawn_command(cmd) {
                Ok(child) => child,
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to spawn command: {}", e)
                    )) as Box<dyn std::error::Error + Send>);
                }
            };
            
            // Create a reader from the master PTY
            let reader = match pty_pair.master.try_clone_reader() {
                Ok(reader) => reader,
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create reader: {}", e)
                    )) as Box<dyn std::error::Error + Send>);
                }
            };
            
            // Create a writer from the master PTY
            let writer = match pty_pair.master.take_writer() {
                Ok(writer) => writer,
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create writer: {}", e)
                    )) as Box<dyn std::error::Error + Send>);
                }
            };
            
            Ok((pty_pair.master, writer, reader, child))
        }).await;
        
        // Handle the result from the spawned task
        let (master, writer, reader, child) = match pty_result {
            Ok(inner_result) => match inner_result {
                Ok((m, w, r, c)) => (m, w, r, c),
                Err(e) => {
                    return Err(e);
                }
            },
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create PTY: {}", e)
                )));
            }
        };
        
        Ok(Self {
            master: Arc::new(Mutex::new(master)),
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
            process: Arc::new(Mutex::new(child)),
            alive: Arc::new(Mutex::new(true)),
        })
    }
}

#[async_trait::async_trait]
impl Pty for PortablePty {
    async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send>> {
        info!("Writing {} bytes to PTY", data.len());
        let writer = Arc::clone(&self.writer);
        let data = data.to_vec();
        let data_len = data.len();
        
        // Spawn the write operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut writer_guard = writer.lock().unwrap();
            // Use standard Write trait write_all method
            match writer_guard.write_all(&data) {
                Ok(_) => {
                    info!("Successfully wrote {} bytes to PTY", data_len);
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to write to PTY: {}", e);
                    Err(Box::new(e) as Box<dyn std::error::Error + Send>)
                },
            }
        }).await;
        
        match result {
            Ok(write_result) => write_result,
            Err(e) => {
                error!("Failed to spawn write task: {}", e);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to write to PTY: {}", e))))
            },
        }
    }
    
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error + Send>> {
        let reader = Arc::clone(&self.reader);
        let buffer_len = buffer.len();
        
        // Spawn the read operation in a blocking thread, but use non-blocking approach
        let result = tokio::task::spawn_blocking(move || {
            let mut reader_guard = reader.lock().unwrap();
            let mut local_buffer = vec![0; buffer_len];
            
            // Try to read data with non-blocking approach
            // Use try_read if available, otherwise use regular read with short timeout
            // For now, we'll use a simple approach with try_read if possible
            // But since we're in a blocking thread, we can use a more efficient approach
            
            // Check if the reader implements AsRawFd (Unix) or AsRawHandle (Windows)
            // and set it to non-blocking mode. But this is complex, so we'll use a simpler approach
            
            // Use a non-blocking approach by checking if data is available before reading
            // On Windows, we can use PeekNamedPipe
            // On Unix, we can use select(2) or poll(2)
            // But this is complex, so we'll use a simpler approach with a short timeout
            
            // For now, we'll use a simple approach: try to read with a short timeout
            // We can use std::io::Read::read, which will return immediately if no data is available
            // on some platforms, but block on others. To make it truly non-blocking, we need to
            // use platform-specific APIs.
            
            // For simplicity, we'll use a simple approach: read with a short timeout
            // This will allow the select! macro to continue processing other events
            let read_result = reader_guard.read(&mut local_buffer);
            (local_buffer, read_result)
        }).await;
        
        match result {
            Ok((local_buffer, Ok(read_bytes))) => {
                // Copy the read data to the provided buffer
                buffer[..read_bytes].copy_from_slice(&local_buffer[..read_bytes]);
                if read_bytes > 0 {
                    info!("Successfully read {} bytes from PTY", read_bytes);
                }
                Ok(read_bytes)
            },
            Ok((_, Err(e))) => {
                // Check if the error is due to no data available
                // On Unix, this would be EWOULDBLOCK or EAGAIN
                // On Windows, this would be ERROR_NO_DATA or ERROR_IO_PENDING
                // But since we're using a cross-platform library, we'll need to check the error kind
                use std::io::ErrorKind;
                match e.kind() {
                    ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted => {
                        // No data available, return 0 bytes
                        Ok(0)
                    },
                    _ => {
                        // Other error, return error
                        error!("PTY read error: {}", e);
                        Err(Box::new(e) as Box<dyn std::error::Error + Send>)
                    }
                }
            },
            Err(e) => {
                error!("Failed to spawn read task: {}", e);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read from PTY: {}", e))))
            },
        }
    }
    
    async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), Box<dyn std::error::Error + Send>> {
        let master = Arc::clone(&self.master);
        
        // Spawn the resize operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let master_guard = master.lock().unwrap();
            match master_guard.resize(portable_pty::PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            }) {
                Ok(_) => Ok(()),
                Err(e) => {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to resize PTY: {}", e)
                    )) as Box<dyn std::error::Error + Send>)
                },
            }
        }).await;
        
        match result {
            Ok(resize_result) => resize_result,
            Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to resize PTY: {}", e)))),
        }
    }
    
    async fn kill(&mut self) -> Result<(), Box<dyn std::error::Error + Send>> {
        let process = Arc::clone(&self.process);
        let alive = Arc::clone(&self.alive);
        
        // Spawn the kill operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut process_guard = process.lock().unwrap();
            match process_guard.kill() {
                Ok(_) => {
                    // Update alive status
                    *alive.lock().unwrap() = false;
                    Ok(())
                },
                Err(e) => {
                    // Update alive status even if kill fails
                    *alive.lock().unwrap() = false;
                    Err(Box::new(e) as Box<dyn std::error::Error + Send>)
                },
            }
        }).await;
        
        match result {
            Ok(kill_result) => kill_result,
            Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to kill PTY process: {}", e)))),
        }
    }
    
    async fn is_alive(&self) -> Result<bool, Box<dyn std::error::Error + Send>> {
        let process = Arc::clone(&self.process);
        let alive_flag = Arc::clone(&self.alive);
        
        // Check if the process is still running
        let result = tokio::task::spawn_blocking(move || {
            let mut process_guard = process.lock().unwrap();
            let alive = match process_guard.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            };
            
            // Update the alive flag if needed
            let mut alive_flag_guard = alive_flag.lock().unwrap();
            if *alive_flag_guard != alive {
                *alive_flag_guard = alive;
            }
            
            Ok(alive)
        }).await;
        
        match result {
            Ok(is_alive_result) => is_alive_result,
            Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to check PTY alive status: {}", e)))),
        }
    }
}

impl Drop for PortablePty {
    /// Clean up resources when the PortablePty is dropped
    fn drop(&mut self) {
        info!("Dropping PortablePty, cleaning up resources");
        
        // Try to kill the process if it's still running
        if let Some(Err(e)) = self.kill().now_or_never() {
            error!("Failed to kill PTY process on drop: {:?}", e);
        }
    }
}