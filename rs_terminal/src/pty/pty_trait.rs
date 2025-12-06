use async_trait::async_trait;

/// Trait for abstracting PTY operations
/// This allows different PTY implementations to be used interchangeably
#[async_trait]
pub trait Pty {
    /// Write data to the PTY
    async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send>>;

    /// Read data from the PTY
    async fn read(&mut self, buffer: &mut [u8])
    -> Result<usize, Box<dyn std::error::Error + Send>>;

    /// Resize the PTY terminal window
    async fn resize(
        &mut self,
        cols: u16,
        rows: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send>>;

    /// Kill the PTY process
    async fn kill(&mut self) -> Result<(), Box<dyn std::error::Error + Send>>;

    /// Check if the PTY process is still alive
    async fn is_alive(&self) -> Result<bool, Box<dyn std::error::Error + Send>>;
}
