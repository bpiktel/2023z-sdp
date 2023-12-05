//! Signal handling

use tokio::signal;
use tracing::info;

// Async functon that ends when a signal is detected
pub async fn shutdown_signal() {
    // Multi-platform CTRL+C signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };

    // Unix-only code
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install termination signal handler")
            .recv()
            .await;
    };

    // Not-unix-only code
    // Never ends, so the only option is CTRL+C
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for one or the other
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}
