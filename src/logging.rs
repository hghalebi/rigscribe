use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initializes the application's logging infrastructure.
///
/// This function sets up two logging layers:
/// 1. A daily rolling file appender in the `logs/` directory.
/// 2. A console output for standard output.
///
/// It uses `tracing_subscriber` to compose these layers with an `EnvFilter`.
///
/// # Returns
///
/// A [`WorkerGuard`] which *must* be assigned to a variable (e.g., `_guard`) in `main`.
/// Dropping this guard will flush any remaining logs and shut down the writer.
pub fn init_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "rigscribe.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer()
        .with_target(true) // Include context (target)
        .with_thread_ids(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .compact(); // Use a more compact format for console if desired, or pretty()

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false); // Disable colors for file

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rigscribe=debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}