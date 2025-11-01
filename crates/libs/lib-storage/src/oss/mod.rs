use std::sync::Arc;
use ali_oss_rs::Client;
use ali_oss_rs::acl::ObjectAclOperations;
use ali_oss_rs::object::{ObjectOperations};
use ali_oss_rs::object_common::GetObjectOptions;
use ali_oss_rs::object_common::{ObjectAcl, PutObjectOptions};
use lib_utils::mime::get_mime_from_bytes;
use tracing::{debug, info};
use crate::config::oss_config;

mod error;
pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct OssClient {
    client: Arc<Client>,
    bucket_name: String,
    public_base: String,
}

impl OssClient {
    pub fn new() -> Self {
        let config = oss_config();

        let client = Client::new(
            &config.OSS_ACCESS_KEY_ID,
            &config.OSS_ACCESS_KEY_SECRET,
            &config.OSS_REGION,
            &config.OSS_ENDPOINT,
        );

        Self {
            client: Arc::new(client),
            bucket_name: config.OSS_BUCKET_NAME.clone(),
            public_base: config.OSS_PUBLIC_BASE.clone(),
        }
    }

    /// --- Load file in OSS and make it public
    pub async fn upload(&self, filename: &str, data: &[u8]) -> Result<String> {
        info!("{:<12} - Uploading file: {}", "OSS", filename);
        debug!("{:<12} - File size: {} bytes", "OSS", data.len());
                
        get_mime_from_bytes(data, filename);

        let put_options = PutObjectOptions {
            content_md5: None,
            ..Default::default()
        };

        self.client
            .put_object_from_buffer(&self.bucket_name, filename, data, Some(put_options))
            .await
            .map_err(|e| Error::UploadError(format!("put_object_from_buffer: {}", e)))?;

        self.client
            .put_object_acl(&self.bucket_name, filename, ObjectAcl::PublicRead, None)
            .await
            .map_err(|e| Error::UploadError(format!("put_object_acl: {}", e)))?;

        info!("{:<12} - File uploaded successfully: {}", "OSS", filename);

        Ok(self.public_url(filename))
    }


    /// --- Download file as bites
    pub async fn download(&self, filename: &str) -> Result<Vec<u8>> {
        info!("{:<12} - Downloading file: {}", "OSS", filename);

        let result = self
            .client
            .get_object_to_buffer(&self.bucket_name, filename, None::<GetObjectOptions>)
            .await
            .map_err(|e| Error::UploadError(format!("get_object_to_buffer: {}", e)))?;

        Ok(result)
    }


    /// --- Delete file
    pub async fn delete(&self, filename: &str) -> Result<()> {
        info!("{:<12} - Deleting file: {}", "OSS", filename);

        self.client
            .delete_object(&self.bucket_name, filename, None)
            .await
            .map_err(|e| Error::UploadError(format!("delete_object: {}", e)))?;
        
        info!("{:<12} - Deleted file: {}", "OSS", filename);

        Ok(())
    }

    /// --- Check if object exists
    pub async fn exists(&self, filename: &str) -> Result<bool> {
        info!("{:<12} - Checking if file exists: {}", "OSS", filename);

        match self.client.head_object(&self.bucket_name, filename, None).await {
            Ok(_) => Ok(true),
            Err(err) => {
                let msg = err.to_string();
                if msg.contains("NoSuchKey") || msg.contains("404") {
                    Ok(false)
                } else {
                    Err(Error::UploadError(format!("head_object: {}", msg)))
                }
            }
        }
    }

    /// --- Create URL for object
    pub fn public_url(&self, filename: &str) -> String {
        info!("{:<12} - Creating public URL: {}", "OSS", filename);

        format!(
            "{}/{}",
            self.public_base.trim_end_matches('/'),
            filename.trim_start_matches('/')
        )
    }
}
