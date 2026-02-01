use crate::model::Result;
use lib_storage::oss::OssClient;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    async fn upload(&self, filename: &str, data: &[u8], mime: &str) -> Result<String>;
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
    async fn upload(&self, filename: &str, data: &[u8], mime: &str) -> Result<String> {
        let url = self.oss.upload(filename, data, mime).await?;
        Ok(url)
    }

    async fn delete_by_url(&self, url: &str) -> Result<()> {
        self.oss.delete_by_url(url).await.map_err(Into::into)
    }
}
