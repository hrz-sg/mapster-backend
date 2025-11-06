use lib_storage::oss::OssClient;
use crate::model::Result;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    async fn upload(&self, filename: &str, data: &[u8]) -> Result<(String, String)>;
    async fn delete_by_url(&self, url: &str) -> Result<()>;
}

pub struct MediaStorageService {
    oss: OssClient,
}

impl MediaStorageService {
    pub fn new() -> Self {
        Self { oss: OssClient::new() }
    }
}

#[async_trait::async_trait]
impl Storage for MediaStorageService {
    async fn upload(&self, filename: &str, data: &[u8]) -> Result<(String, String)> {
        let (url, mime) = self.oss.upload(filename, data).await?;
        Ok((url, mime))
    }

    async fn delete_by_url(&self, url: &str) -> Result<()> {
        self.oss.delete_by_url(url).await.map_err(Into::into)
    }
}