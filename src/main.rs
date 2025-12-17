use log::{debug, info};
use tower_http::catch_panic::CatchPanicLayer;
use rust_aws_s3_downloader::config::graceful_shutdown::graceful_shutdown_handler;
use rust_aws_s3_downloader::constant::constants::{API_SERVER_HOST_DEFAULT, API_SERVER_HOST_ENV_VAR, API_SERVER_PORT_DEFAULT, API_SERVER_PORT_ENV_VAR, LOGGING_CONFIG_FILE_DEFAULT, LOGGING_CONFIG_FILE_ENV_VAR};
use rust_aws_s3_downloader::controller::download_controller::*;
use rust_aws_s3_downloader::controller::health_controller::*;

/// App main function (multi-thread implemented by tokio dependency)
///
/// **important:** if LOG4RS_CONFIG_FILE environment variable is not defined,
/// logger configuration will be read from logging_config.yaml file from project´s root
#[tokio::main]
async fn main() {
    let log_config_file = std::env::var(LOGGING_CONFIG_FILE_ENV_VAR).unwrap_or(String::from(LOGGING_CONFIG_FILE_DEFAULT));
    log4rs::init_file(log_config_file, Default::default()).unwrap();

    start_api_server().await;
}

/// Starts API server
async fn start_api_server() {
    info!("Axum server - starting...");

    let api_server_host = std::env::var(API_SERVER_HOST_ENV_VAR).unwrap_or(String::from(API_SERVER_HOST_DEFAULT));
    debug!("start_api_server - api server host retrieved OK");

    let api_server_port = std::env::var(API_SERVER_PORT_ENV_VAR).unwrap_or(String::from(API_SERVER_PORT_DEFAULT));
    debug!("start_api_port - api server host retrieved OK");

    let api_server_address = format!("{api_server_host}:{api_server_port}");
    info!("Axum server - listening on: {api_server_address}");

    let api_tcp_listener = tokio::net::TcpListener::bind(api_server_address)
        .await
        .unwrap();
    let _ = axum::serve(
        api_tcp_listener,
        HealthController::config_endpoints()
            .merge(DownloadController::config_endpoints())
            .layer(CatchPanicLayer::new())
            .into_make_service(),
    )
    .with_graceful_shutdown(graceful_shutdown_handler())
    .await;
}
