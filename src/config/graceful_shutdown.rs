use log::info;

/// Server API handler for graceful shutdown (windows version)
/// based on official Axum sample:
/// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
pub async fn graceful_shutdown_handler() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("graceful_shutdown_handler - failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("graceful_shutdown_handler - failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("graceful_shutdown_handler - by Ctrl+C handler");
            graceful_shutdown().await
        },
        _ = terminate => {
            info!("graceful_shutdown_handler - by signal handler");
            graceful_shutdown().await
        },
    }
}

/// Performs graceful shutdown common steps for windows and unix environments
async fn graceful_shutdown() {
    info!("graceful_shutdown - app graceful_shutdown - starting...");
    info!("graceful_shutdown - app graceful_shutdown - done");
}
