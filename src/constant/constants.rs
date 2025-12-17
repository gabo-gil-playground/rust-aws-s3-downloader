/// logging configuration file environment variable name
pub const LOGGING_CONFIG_FILE_ENV_VAR: &str = "LOG4RS_CONFIG_FILE";
/// logging configuration file default value
pub const LOGGING_CONFIG_FILE_DEFAULT: &str = "logging_config.yaml";

/// API server host environment variable name
pub const API_SERVER_HOST_ENV_VAR: &str = "API_SERVER_HOST";
/// API server port environment variable name
pub const API_SERVER_PORT_ENV_VAR: &str = "API_SERVER_PORT";
/// API server host default value
pub const API_SERVER_HOST_DEFAULT: &str = "0.0.0.0";
/// API server port default value
pub const API_SERVER_PORT_DEFAULT: &str = "8097";
/// server running status message
pub const SERVER_RUNNING_STATUS: &str = "server is running";
/// delete entity status message
pub const DELETE_OK_STATUS: &str = "deleted ok";

/// API Health-check main path
pub const API_HEALTH_CHECK_PATH: &str = "/health";

/// API main path
pub const API_MAIN_PATH: &str = "/api/v1";

/// API Download main path
pub const API_DOWNLOAD_MAIN_PATH: &str = "/api/v1/download";
pub const API_DOWNLOAD_ALL_AS_ZIP_PATH: &str = "/zip";

/// AWS S3 max files supported environment variable and default vlaue
pub const AWS_S3_MAX_FILE_QUANTITY_ENV_VAR: &str = "AWS_S3_MAX_FILE_QUANTITY";
pub const AWS_S3_MAX_FILE_QUANTITY_DEFAULT: &str = "100";

/// AWS S3 max file size supported (in bytes) environment variable and default vlaue
pub const AWS_S3_MAX_FILE_SIZE_BYTES_ENV_VAR: &str = "AWS_S3_MAX_FILE_SIZE_BYTES";
pub const AWS_S3_MAX_FILE_SIZE_BYTES_DEFAULT: &str = "2097152"; // ((bytes * 1024 = KB) * 1024 = MB)

/// Unit test cases
#[cfg(test)]
mod tests {}
