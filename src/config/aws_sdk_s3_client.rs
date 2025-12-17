use async_trait::async_trait;
use std::sync::Arc;

use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::Client;

use log::debug;

/// AWS ASK S3 client trait
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AwsSdkS3ClientTrait {
    /// Creates a new [Client] AWS SDK Client
    async fn create_aws_sdk_client(&self) -> Client;
}

#[derive(Default)]
pub struct AwsSdkS3Client {}

/// AWS SDK S3 client implementation logic
#[async_trait]
impl AwsSdkS3ClientTrait for AwsSdkS3Client {
    /// Creates a new [Client] AWS SDK Client
    async fn create_aws_sdk_client(&self) -> Client {
        debug!("create_aws_sdk_client - start");
        let aws_sdk_configuration = aws_config::load_defaults(BehaviorVersion::latest()).await;

        debug!("create_aws_sdk_client - done");
        Client::new(&aws_sdk_configuration)
    }
}

/// AWS SDK S3 client trait dyn type
pub type DynAwsSdkS3Client = Arc<dyn AwsSdkS3ClientTrait + Send + Sync>;
