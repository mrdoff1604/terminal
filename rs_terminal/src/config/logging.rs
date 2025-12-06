use tracing_subscriber;

/// Initialize logging configuration
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter("rs_terminal=debug")
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
