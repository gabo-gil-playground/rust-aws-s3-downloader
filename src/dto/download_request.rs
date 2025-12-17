use serde::{Deserialize, Serialize};

/// Download request struct
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct DownloadRequest {
    /// S3 bucket
    pub bucket_name: String,
    /// S3 folder full path
    pub full_path: String,
}

/// Unit test cases
#[cfg(test)]
mod tests {
}
