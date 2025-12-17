use crate::config::aws_sdk_s3_client::{AwsSdkS3Client, DynAwsSdkS3Client};
use crate::enums::common_error::CommonError;
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use axum::body::Bytes;
use log::{debug, error, warn};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::task::JoinSet;
use crate::constant::constants::{AWS_S3_MAX_FILE_QUANTITY_DEFAULT, AWS_S3_MAX_FILE_QUANTITY_ENV_VAR, AWS_S3_MAX_FILE_SIZE_BYTES_DEFAULT, AWS_S3_MAX_FILE_SIZE_BYTES_ENV_VAR};

/// AWS SDK S3 client
/// important: libs can't export test attributes so we should use debug_assertions instead of test macro for child crates
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AwsSdkS3ServiceTrait {
    /// Adds S3 object by [String] bucket name, [String] path, [String] s3 key and [Bytes] content
    /// Returns a [String] with added S3 file key value
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn add_s3_object(
        &self,
        bucket_name: String,
        path: String,
        s3_key: String,
        s3_key_content: &Bytes,
    ) -> Result<String, CommonError>;

    /// Gets [(String, Vec<u8>)] S3 key value and stream content by [String] bucket name,
    /// [String] path and [String] s3 key
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_object(
        &self,
        bucket_name: String,
        path: String,
        s3_key: String,
    ) -> Result<(String, Vec<u8>), CommonError>;

    /// Gets [Vec<String>] S3 key list by [String] bucket name and [String] path
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_object_key_list(
        &self,
        bucket_name: String,
        path: String,
    ) -> Result<Vec<String>, CommonError>;

    /// Gets [(String, Vec<u8>)] S3 objects keys and contents by [String] bucket name and [String] path
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_objects_by_path(
        &self,
        bucket_name: String,
        path: String,
    ) -> Result<Vec<(String, Vec<u8>)>, CommonError>;

    /// Gets [(Vec<(String, Vec<u8>)>, Vec<String>)] S3 objects keys and contents + not found keys
    /// by [String] bucket name, [String] path and [Vec<String>] S3 key list
    /// Returns a [CommonError] if result is empty or S3 throws any error
    #[allow(clippy::type_complexity)] // avoid define the result as a type (suggested by clippy)
    async fn get_s3_objects_by_keys(
        &self,
        bucket_name: String,
        path: String,
        s3_keys: Vec<String>,
    ) -> Result<(Vec<(String, Vec<u8>)>, Vec<String>), CommonError>;
}

/// AWS SDK S3 service implementation struct
#[derive(Clone)]
pub struct AwsSdkS3Service {
    aws_sdk_s3_client: DynAwsSdkS3Client,
    aws_sdk_s3_max_file_qty: usize,
    aws_sdk_s3_max_file_size: i64,
}

/// default initialization
impl Default for AwsSdkS3Service {
    fn default() -> Self {
        AwsSdkS3Service {
            aws_sdk_s3_client: Arc::new(AwsSdkS3Client::default()) as DynAwsSdkS3Client,
            aws_sdk_s3_max_file_qty: get_env_var_as_usize(AWS_S3_MAX_FILE_QUANTITY_ENV_VAR, AWS_S3_MAX_FILE_QUANTITY_DEFAULT),
            aws_sdk_s3_max_file_size: get_env_var_as_i64(AWS_S3_MAX_FILE_SIZE_BYTES_ENV_VAR, AWS_S3_MAX_FILE_SIZE_BYTES_DEFAULT),
        }
    }
}

/// AWS S3 client implementation logic
#[async_trait]
impl AwsSdkS3ServiceTrait for AwsSdkS3Service {
    /// Adds S3 object by [String] bucket name, [String] path, [String] s3 key and [Bytes] content
    /// Returns a [String] with added S3 file key value
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn add_s3_object(
        &self,
        bucket_name: String,
        path: String,
        s3_key: String,
        s3_key_content: &Bytes,
    ) -> Result<String, CommonError> {
        debug!("add_s3_object - start");

        if bucket_name.is_empty()
            || path.is_empty()
            || s3_key.is_empty()
            || s3_key_content.is_empty()
        {
            error!("add_s3_object - empty bucket name, path, s3 key or content - bucket name: {bucket_name}");
            error!("add_s3_object - empty bucket name, path, s3 key or content - path: {path:}");
            error!("add_s3_object - empty bucket name, path, s3 key or content - s3 key: {s3_key}");
            return Err(CommonError::NO_VALID_INPUT_OR_PARAMETER);
        }

        debug!("add_s3_object - upload start - s3 key: {}", &s3_key);
        let client_s3 = self.aws_sdk_s3_client.create_aws_sdk_client().await;

        match client_s3
            .put_object()
            .bucket(&bucket_name)
            .key(format!("{}/{}", sanitize_path(path), &s3_key))
            .body(ByteStream::from(s3_key_content.clone()))
            .send()
            .await
        {
            Ok(_) => {
                debug!("add_s3_object - upload completed - s3 key: {s3_key}");
                debug!("add_s3_object - done");
                Ok(s3_key)
            }
            Err(_) => {
                error!("add_s3_object - upload error - bucket name: {bucket_name}");
                error!("add_s3_object - upload error - s3 key: {s3_key}");
                Err(CommonError::AWS_ACCESS_ERROR)
            }
        }
    }

    /// Gets [(String, Vec<u8>)] S3 key value and stream content by [String] bucket name,
    /// [String] path and [String] s3 key
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_object(
        &self,
        bucket_name: String,
        path: String,
        s3_key: String,
    ) -> Result<(String, Vec<u8>), CommonError> {
        debug!("get_s3_object - start");
        debug!("get_s3_object - bucket name: {}", &bucket_name);
        debug!("get_s3_object - s3 path: {}", &path);
        debug!("get_s3_object - s3 key: {}", &s3_key);

        let aws_sdk_client = self.aws_sdk_s3_client.create_aws_sdk_client().await;
        match get_s3_object_content(
            aws_sdk_client,
            bucket_name.clone(),
            path.clone(),
            s3_key.clone(),
        )
        .await
        {
            Ok(s3_object) => {
                debug!("get_s3_object - done");
                Ok(s3_object)
            }
            Err(_) => {
                error!("get_s3_object - s3 object not found - bucket name: {bucket_name}");
                error!("get_s3_object - s3 object not found - path: {path}");
                error!("get_s3_object - s3 object not found - s3 key: {s3_key}");
                Err(CommonError::AWS_ACCESS_ERROR)
            }
        }
    }

    /// Gets [Vec<String>] S3 key list by [String] bucket name and [String] path
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_object_key_list(
        &self,
        bucket_name: String,
        path: String,
    ) -> Result<Vec<String>, CommonError> {
        debug!("get_s3_object_key_list - start");
        debug!("get_s3_object_key_list - bucket name: {}", &bucket_name);
        debug!("get_s3_object_key_list - path: {}", &path);

        let aws_sdk_client = self.aws_sdk_s3_client.create_aws_sdk_client().await;
        match aws_sdk_client
            .list_objects()
            .bucket(&bucket_name)
            .prefix(sanitize_path(path.clone()))
            .send()
            .await
        {
            Ok(s3_object_list) => {
                let s3_object_key_list: Vec<String> = s3_object_list
                    .contents
                    .unwrap_or_default()
                    .iter()
                    .filter(|s3_object| s3_object.key.is_some())
                    .filter(|s3_object| s3_object.size.unwrap_or_default() < self.aws_sdk_s3_max_file_size)
                    .map(|s3_object| s3_object.key.clone().unwrap_or_default())
                    .map(|s3_key| {
                        String::from(
                            s3_key
                                .strip_prefix(&format!("{}/", &sanitize_path(path.clone())))
                                .unwrap(),
                        )
                    })
                    .filter(|s3_key| !s3_key.contains("/"))
                    .collect();

                if s3_object_key_list.len() > self.aws_sdk_s3_max_file_qty {
                    error!(
                        "get_s3_object_key_list - s3 object key list is greater than configured maximum file quantity - bucket name: {bucket_name}"
                    );
                    error!(
                        "get_s3_object_key_list - s3 object key list is greater than configured maximum file quantity - path: {path}"
                    );
                    return Err(CommonError::AWS_ACCESS_ERROR);
                }

                debug!("get_s3_object_key_list - done");
                Ok(s3_object_key_list)
            }
            Err(s3_object_error) => {
                error!("get_s3_object_key_list - s3 object key list not found - error: {s3_object_error}");
                error!(
                    "get_s3_object_key_list - s3 object key list not found - bucket name: {bucket_name}"
                );
                error!(
                    "get_s3_object_key_list - s3 object key list not found - path: {path}"
                );
                Err(CommonError::AWS_ACCESS_ERROR)
            }
        }
    }

    /// Gets [(String, Vec<u8>)] S3 objects keys and contents by [String] bucket name and [String] path
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_objects_by_path(
        &self,
        bucket_name: String,
        path: String,
    ) -> Result<Vec<(String, Vec<u8>)>, CommonError> {
        debug!("get_s3_objects_by_path - start");
        debug!("get_s3_objects_by_path - bucket name: {}", &bucket_name);
        debug!("get_s3_objects_by_path - path: {}", &path);

        match self
            .get_s3_object_key_list(bucket_name.clone(), path.clone())
            .await
        {
            Ok(s3_object_key_list_values) => {
                let aws_sdk_client = self.aws_sdk_s3_client.create_aws_sdk_client().await;

                let mut tokio_join_set = JoinSet::new();
                let mut s3_object_key_found_list = Vec::new();

                s3_object_key_list_values.iter().for_each(|s3_key| {
                    tokio_join_set.spawn(get_s3_object_content(
                        aws_sdk_client.clone(),
                        bucket_name.clone(),
                        path.clone(),
                        s3_key.clone(),
                    ));
                });

                while let Some(result) = tokio_join_set.join_next().await {
                    s3_object_key_found_list.push(result.unwrap().unwrap_or_default());
                }

                debug!("get_s3_objects_by_path - done");
                Ok(s3_object_key_found_list)
            }
            Err(s3_object_key_list_error) => {
                error!("get_s3_objects_by_path - s3 objects not found - error: {s3_object_key_list_error}");
                error!("get_s3_objects_by_path - s3 objects not found - bucket name: {bucket_name}");
                error!("get_s3_objects_by_path - s3 objects not found - path: {path}");
                Err(CommonError::AWS_ACCESS_ERROR)
            }
        }
    }

    /// Gets [(Vec<(String, Vec<u8>)>, Vec<String>)] S3 objects keys and contents + not found keys
    /// by [String] bucket name, [String] path and [Vec<String>] S3 key list
    /// Returns a [CommonError] if result is empty or S3 throws any error
    async fn get_s3_objects_by_keys(
        &self,
        bucket_name: String,
        path: String,
        s3_keys: Vec<String>,
    ) -> Result<(Vec<(String, Vec<u8>)>, Vec<String>), CommonError> {
        debug!("get_s3_objects_by_keys - start");
        debug!("get_s3_objects_by_keys - bucket name: {}", &bucket_name);
        debug!("get_s3_objects_by_keys - path: {}", &path);
        debug!("get_s3_objects_by_keys - s3 keys: {:?}", &s3_keys);

        match self
            .get_s3_object_key_list(bucket_name.clone(), path.clone())
            .await
        {
            Ok(s3_object_key_list) => {
                let client_s3 = self.aws_sdk_s3_client.create_aws_sdk_client().await;

                let mut s3_object_key_found_list = Vec::new();
                let mut s3_object_key_not_found_list = Vec::new();
                let mut tokio_join_set = JoinSet::new();

                s3_object_key_list.iter().for_each(|s3_key| {
                    if s3_keys.contains(s3_key) {
                        tokio_join_set.spawn(get_s3_object_content(
                            client_s3.clone(),
                            bucket_name.clone(),
                            path.clone(),
                            s3_key.clone(),
                        ));
                    } else {
                        warn!("get_s3_objects_by_keys - s3 key not found: {}", &s3_key);
                        s3_object_key_not_found_list.push(s3_key.clone());
                    }
                });

                while let Some(result) = tokio_join_set.join_next().await {
                    s3_object_key_found_list.push(result.unwrap().unwrap_or_default());
                }

                debug!("get_s3_objects_by_keys - done");
                Ok((s3_object_key_found_list, s3_object_key_not_found_list))
            }
            Err(s3_object_key_list_error) => {
                error!("get_s3_objects_by_keys - s3 objects not found - error: {s3_object_key_list_error}");
                error!("get_s3_objects_by_keys - s3 objects not found - bucket name: {bucket_name}");
                error!("get_s3_objects_by_keys - s3 objects not found - path: {path}");
                debug!("get_s3_objects_by_keys - s3 objects not found - s3 keys: {s3_keys:?}");
                Err(CommonError::AWS_ACCESS_ERROR)
            }
        }
    }
}

/// Gets [(String, Vec<u8>)] S3 key value and stream content by [Client] AWS SDK client,
/// [String] bucket name, [String] path and [String] s3 key
/// Returns a [CommonError] if result is empty or S3 throws any error
async fn get_s3_object_content(
    aws_sdk_client: Client,
    bucket_name: String,
    path: String,
    s3_key: String,
) -> Result<(String, Vec<u8>), CommonError> {
    debug!("get_s3_object_content - start");

    match aws_sdk_client
        .get_object()
        .bucket(bucket_name.clone())
        .key(format!(
            "{}/{}",
            sanitize_path(path.clone()),
            s3_key.clone()
        ))
        .send()
        .await
    {
        Ok(s3_object_content) => {
            debug!("get_s3_object_content - s3 key found");
            let mut content_as_vec = Vec::new();
            let _ = s3_object_content
                .body
                .into_async_read()
                .read_to_end(&mut content_as_vec)
                .await;

            debug!("get_s3_object_content - done");
            Ok((s3_key, content_as_vec))
        }
        Err(s3_object_error) => {
            error!("get_s3_object_content - s3 object not found - error: {s3_object_error}",);
            error!("get_s3_object_content - s3 object not found - bucket name: {bucket_name}");
            error!("get_s3_object_content - s3 object not found - path: {path}");
            error!("get_s3_object_content - s3 object not found - s3 key: {s3_key}");
            Err(CommonError::AWS_ACCESS_ERROR)
        }
    }
}

/// Gets a [String] sanitized path by [String] S3 path
/// Important: removes start and end slashes to avoid included nested folders as part of
/// implemented S3 operations (like read, add, delete, copy, etc.)
fn sanitize_path(mut path_to_sanitize: String) -> String {
    if path_to_sanitize.starts_with('/') {
        path_to_sanitize = path_to_sanitize[1..].to_string();
    }

    if path_to_sanitize.ends_with('/') {
        path_to_sanitize = path_to_sanitize[..(path_to_sanitize.len() - 1)].to_string();
    }

    path_to_sanitize
}

/// Gets [i64] value by [&str] environment variable name and [&str] environment variable default value
fn get_env_var_as_i64(env_var_name: &str, env_var_default: &str) -> i64 {
    let value = std::env::var(env_var_name).unwrap_or(String::from(env_var_default));
    return value.parse().unwrap_or_default()
}

/// Gets [usize] value by [&str] environment variable name and [&str] environment variable default value
fn get_env_var_as_usize(env_var_name: &str, env_var_default: &str) -> usize {
    let value = std::env::var(env_var_name).unwrap_or(String::from(env_var_default));
    return value.parse().unwrap_or_default()
}

/// AWS SDK S3 service trait dyn type
pub type DynAwsSdkS3Service = Arc<dyn AwsSdkS3ServiceTrait + Send + Sync>;
