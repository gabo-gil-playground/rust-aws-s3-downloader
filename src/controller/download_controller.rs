use std::io::Cursor;
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::{
    Json, Router,
    response::IntoResponse,
    routing::post,
};
use axum::body::Body;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use tokio_util::io::ReaderStream;
use crate::constant::constants::{API_DOWNLOAD_ALL_AS_ZIP_PATH, API_DOWNLOAD_MAIN_PATH};
use crate::dto::download_request::DownloadRequest;
use crate::service::download_service::{DownloadService, DynDownloadService};

/// Download controller
pub trait DownloadControllerTrait {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router;
}

/// Download controller implementation struct
pub struct DownloadController {}

/// Download controller implementation logic
impl DownloadControllerTrait for DownloadController {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router {
        let download_service = Arc::new(DownloadService::default()) as DynDownloadService;
        Router::new()
            .nest(API_DOWNLOAD_MAIN_PATH, create_routes())
            .with_state(download_service)
    }
}

/// Creates Foo routes
fn create_routes() -> Router<DynDownloadService> {
    Router::new()
        .route(API_DOWNLOAD_ALL_AS_ZIP_PATH, post(map_download))
}

/// Maps download end-point
async fn map_download(
    State(download_service): State<DynDownloadService>,
    download_request: Json<DownloadRequest>,
) -> impl IntoResponse {
    match download_service.download_files(download_request.0.bucket_name, download_request.0.full_path).await {
        Ok(export_file_content) => {
            let headers = create_export_headers(&export_file_content.0);
            let body = Body::from_stream(ReaderStream::new(Cursor::new(export_file_content.1)));
            (headers, body).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Creates a new [HeaderMap] with [CONTENT_TYPE] and [CONTENT_DISPOSITION] headers based on [&str] filename
pub fn create_export_headers(filename: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/zip; charset=utf-8"));
    header_map.insert(CONTENT_DISPOSITION, HeaderValue::from_str(&format!("attachment; filename=\"{filename}\"")).unwrap());

    header_map
}

/// Unit test cases
#[cfg(test)]
mod tests {
}
