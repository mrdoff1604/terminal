use std::sync::{Arc, Mutex};
use tracing::{info, error, debug};
use futures_util::future::FutureExt;

/// PtyManager manages the PTY process and provides async methods for communication
pub struct PtyManager {
    /// The PTY master for writing to the terminal
    master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    /// The reader for reading from the terminal
    reader: Arc<Mutex<Box<dyn std::io::Read + Send>>>,
    /// The process handle
    process: Arc<Mutex<std::process::Child>>,
}

impl PtyManager {
    /// Create a new PtyManager instance
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send>> {
        // Spawn the PTY creation in a blocking thread since it's a blocking operation
        let (master, reader, process) = tokio::task::spawn_blocking(move || {
            // Get the default PTY pair
            let pty_pair = portable_pty::Pair::open().unwrap();
            
            // Create a command to run in the PTY
            let mut cmd = portable_pty::CommandBuilder::new(
                if cfg!(target_os = "windows") {
                    "cmd.exe"
                } else {
                    "/bin/bash"
                }
            );
            
            // Spawn the process
            let child = pty_pair.slave.spawn_command(cmd).unwrap();
            
            // Get the reader for the PTY
            let reader = pty_pair.master.try_clone_reader().unwrap();
            
            (Box::new(pty_pair.master) as Box<dyn portable_pty::MasterPty + Send>, Box::new(reader) as Box<dyn std::io::Read + Send>, child)
        }).await.map_err(|e| format!("Failed to spawn PTY creation thread: {}", e))?;
        
        Ok(Self {
            master: Arc::new(Mutex::new(master)),
            reader: Arc::new(Mutex::new(reader)),
            process: Arc::new(Mutex::new(process)),
        })
    }
    
    /// Write data to the PTY
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send>> {
        let master = Arc::clone(&self.master);
        let data = data.to_vec();
        
        // Spawn the write operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut master_guard = master.lock().unwrap();
            master_guard.write_all(&data)?;
            Ok(())
        }).await;
        
        match result {
            Ok(write_result) => write_result,
            Err(e) => Err(format!("Failed to write to PTY: {}", e).into()),
        }
    }
    
    /// Read data from the PTY
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error + Send>> {
        let reader = Arc::clone(&self.reader);
        let buffer_len = buffer.len();
        
        // Spawn the read operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut reader_guard = reader.lock().unwrap();
            let mut local_buffer = vec![0; buffer_len];
            let read_bytes = reader_guard.read(&mut local_buffer)?;
            Ok((local_buffer, read_bytes))
        }).await;
        
        match result {
            Ok(read_result) => {
                let (local_buffer, read_bytes) = read_result?;
                // Copy the read data to the provided buffer
                buffer[..read_bytes].copy_from_slice(&local_buffer[..read_bytes]);
                Ok(read_bytes)
            },
            Err(e) => Err(format!("Failed to read from PTY: {}", e).into()),
        }
    }
    
    /// Resize the PTY
    pub async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), Box<dyn std::error::Error + Send>> {
        let master = Arc::clone(&self.master);
        
        // Spawn the resize operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut master_guard = master.lock().unwrap();
            master_guard.resize(portable_pty::Size {
                rows,
                cols,
            })?;
            Ok(())
        }).await;
        
        match result {
            Ok(resize_result) => resize_result,
            Err(e) => Err(format!("Failed to resize PTY: {}", e).into()),
        }
    }
    
    /// Kill the PTY process
    pub async fn kill(&mut self) -> Result<(), Box<dyn std::error::Error + Send>> {
        let process = Arc::clone(&self.process);
        
        // Spawn the kill operation in a blocking thread
        let result = tokio::task::spawn_blocking(move || {
            let mut process_guard = process.lock().unwrap();
            process_guard.kill()?;
            Ok(())
        }).await;
        
        match result {
            Ok(kill_result) => kill_result,
            Err(e) => Err(format!("Failed to kill PTY process: {}", e).into()),
        }
    }
}

impl Drop for PtyManager {
    /// Clean up resources when the PtyManager is dropped
    fn drop(&mut self) {
        info!("Dropping PtyManager, cleaning up resources");
        
        // Try to kill the process if it's still running
        if let Err(e) = self.kill().now_or_never() {
            error!("Failed to kill PTY process on drop: {:?}", e);
        }
    }
}