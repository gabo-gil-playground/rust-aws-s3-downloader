use std::io::{Cursor, Write};
use std::sync::Arc;

use async_trait::async_trait;
use crate::service::aws_sdk_s3_service::{AwsSdkS3Service, DynAwsSdkS3Service};
use log::{error, info};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

/// Download service
#[async_trait]
pub trait DownloadServiceTrait {
    /// Gets [(String, Vec<u8>)] zip file name and content with all downloaded files from Amazon S3
    /// by [String] S3 bucket name and [String] S3 path values
    /// Returns [()] generic error if download flow fails
    /// IMPORTANT:
    /// - maximum file quantity to be downloaded should be configured (please, check constants.rs)
    /// - maximum file supported size should be configured (please, check constants.rs)
    async fn download_files(&self, s3_bucket: String, s3_path: String) -> Result<(String, Vec<u8>), ()>;
}

/// Download service implementation struct
pub struct DownloadService {
    aws_s3_service: DynAwsSdkS3Service,
}

/// default initialization
impl Default for DownloadService {
    fn default() -> Self {
        DownloadService {
            aws_s3_service: Arc::new(AwsSdkS3Service::default()) as DynAwsSdkS3Service,
        }
    }
}

/// Download service implementation logic
#[async_trait]
impl DownloadServiceTrait for DownloadService {
    /// Gets [(String, Vec<u8>)] zip file name and content with all downloaded files from Amazon S3
    /// by [String] S3 bucket name and [String] S3 path values
    /// Returns [()] generic error if download flow fails
    /// IMPORTANT:
    /// - maximum file quantity to be downloaded should be configured (please, check constants.rs)
    /// - maximum file supported size should be configured (please, check constants.rs)
    async fn download_files(&self, s3_bucket: String, s3_path: String) -> Result<(String, Vec<u8>), ()> {
        info!("download_files - start");
        match self.aws_s3_service.get_s3_objects_by_path(s3_bucket.clone(), s3_path.clone()).await {
            Ok(s3_files) => {
                info!("download_files - download files completed - s3 bucket: {s3_bucket}");
                info!("download_files - download files completed - s3 path: {s3_path}");
                info!("download_files - download files completed - s3 files total: {}", s3_files.len());

                info!("download_files - download files completed - create zip file - start");
                let mut zip_content = vec![];
                let mut zip_writer = ZipWriter::new(Cursor::new(&mut zip_content));

                for s3_file in s3_files {
                    zip_writer.start_file(s3_file.0, SimpleFileOptions::default()).unwrap();
                    zip_writer.write_all(&s3_file.1).unwrap();
                }

                zip_writer.finish().unwrap();
                info!("download_files - download files completed - create zip file - done");

                info!("download_files - done");
                Ok((String::from("s3-export.zip"), zip_content.to_vec()))
            }
            Err(_) => {
                error!("download_files - download error - can't get files from s3 bucket: {s3_bucket}");
                error!("download_files - download error - can't get files from s3 path: {s3_path}");
                Err(())
            }
        }
    }
}

/// Download service trait for API router state (based on Rust samples for Axum DI)
pub type DynDownloadService = Arc<dyn DownloadServiceTrait + Send + Sync>;

/// Unit test cases
#[cfg(test)]
mod tests {}
