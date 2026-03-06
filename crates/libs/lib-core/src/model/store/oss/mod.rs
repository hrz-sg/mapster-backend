use std::ops::Range;
use std::sync::Arc;

use ali_oss_rs::multipart::MultipartUploadsOperations;
use ali_oss_rs::multipart_common::{CompleteMultipartUploadRequest, CompleteMultipartUploadResult, ListPartsResult, UploadPartRequest, UploadPartResult};
use ali_oss_rs::object::ObjectOperations;
use ali_oss_rs::object_common::{DeleteMultipleObjectsConfig, DeleteMultipleObjectsResult, DeleteObjectResult, ObjectMetadata};
use ali_oss_rs::presign::SignedOssRequest;
use ali_oss_rs::Client;
use ali_oss_rs::request::{OssRequest, RequestMethod};
use serde::{Deserialize, Serialize};

mod error;
pub use error::{Error, Result};
use lib_utils::file::get_ext;

use crate::model::base::ids::generate_id_for_table;

pub struct Bucket {
    client: Arc<Client>,
    bucket_name: String,
    pub(crate) public_base: String, // TODO: need to create something better
}

impl Clone for Bucket {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            bucket_name: self.bucket_name.clone(),
            public_base: self.public_base.clone(),
        }
    }
}

impl Bucket {
    pub fn new(
        bucket_name: String, 
        public_base: String
    ) -> Self {
        let client = Arc::new(Client::from_env());
        Self { client, bucket_name, public_base }
    }

    pub fn presigned_url(
        &self,
        object_key: &str,
        mime: &str,
    ) -> SignedOssRequest {

        let req = OssRequest::new()
            .method(RequestMethod::Put)
            .bucket(&self.bucket_name)
            .object(object_key)
            .add_header("content-type", mime);

        self.client.presign_raw_request(req)
    }

    pub fn presigned_url_for_part(
        &self,
        object_key: &str,
        upload_id: &str,
        part_number: u32,
        mime: &str,
    ) -> SignedOssRequest {

        let req = OssRequest::new()
            .method(RequestMethod::Put)
            .bucket(&self.bucket_name)
            .object(object_key)
            .add_query("uploadId", upload_id)
            .add_query("partNumber", part_number.to_string())
            .add_header("content-type", mime);

        self.client.presign_raw_request(req)
    }

    pub async fn presigned_url_multipart(
        &self,
        object_key: &str,
        mime: &str,
        part_count: u32,
        expire_sec: u32,
    ) -> Result<(String, Vec<SignedOssRequest>)> {

        // -- Init presigned 
        let init_res = self.client.initiate_multipart_uploads(
            &self.bucket_name,
            &object_key,
            None
        ).await?;

        let upload_id = init_res.upload_id;

        // -- Generate presigned URL for each part
        let mut urls = Vec::new();
        for part_num in 1..=part_count {
            let mut req = OssRequest::new();

            req = req
                .method(RequestMethod::Put)
                .bucket(&self.bucket_name)
                .object(object_key)
                .add_query("partNumber", part_num.to_string())
                .add_query("uploadId", upload_id.clone())
                .add_header("content-type", mime)
                .add_query("x-oss-expires", expire_sec.to_string());

            let signed = self.client.presign_raw_request(req);
            urls.push(signed);
        }
        Ok((upload_id, urls))
    }

    pub async fn list_parts(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<ListPartsResult> {
        let res = self.client.list_parts(&self.bucket_name, object_key, upload_id, None).await?;

        Ok(res)
    }

    pub async fn complete_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<String> {

        // Convert parts into SDK parts
        let sdk_parts: Vec<(u32, String)> = parts
            .into_iter()
            .map(|p| (p.part_number, p.etag))
            .collect();

        let data = CompleteMultipartUploadRequest {
            upload_id: upload_id.to_string(),
            parts: sdk_parts,
        };

        let result = self.client.complete_multipart_uploads(&self.bucket_name, object_key, data, None).await?;

        let etag = match result {
            CompleteMultipartUploadResult::ApiResponse(resp) => resp.etag,
            CompleteMultipartUploadResult::CallbackResponse(body) => body
        };

        Ok(etag)
    }

    pub async fn upload_part_from_file(
        &self,
        object_key: &str,
        file_path: &str,
        range: Range<u64>,
        params: UploadPartRequest,
    ) -> Result<UploadPartResult> {
        let res = self.client.upload_part_from_file(&self.bucket_name, object_key, file_path, range, params).await?;
        Ok(res)
    }

    pub async fn abort_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<()> {
        self.client.abort_multipart_uploads(&self.bucket_name, object_key, upload_id).await?;
        Ok(())
    }

    pub async fn exists(
        &self,
        object_key: &str,
    ) -> Result<bool> {
        let res = self.client.exists(&self.bucket_name, object_key, None).await?;
        Ok(res)
    }
    
    pub async fn head_object(
        &self,
        object_key: &str,
    ) -> Result<ObjectMetadata> {
        let res = self.client.head_object(&self.bucket_name, object_key, None).await?;
        Ok(res)
    }

    pub async fn delete(
        &self,
        object_key: &str,
    ) -> Result<DeleteObjectResult> {
        let res = self.client.delete_object(&self.bucket_name, object_key, None).await?;
        Ok(res)
    }
    
    pub async fn delete_many(
        &self,
        object_keys: &[&str],
    ) -> Result<DeleteMultipleObjectsResult> {

        let config = DeleteMultipleObjectsConfig::FromKeys(object_keys);

        let res = self.client.delete_multiple_objects(&self.bucket_name, config).await?;
        
        Ok(res)
    }
}

pub fn build_object_key(
    entity: &str,
    entity_id: &str,
    filename: &str,
) -> String {
    let ext = get_ext(filename);

    let file_id = generate_id_for_table(&format!("{}_media", entity));

    format!("{}/{}/{}.{}", entity, entity_id, file_id, ext)
}

#[derive(Serialize, Deserialize)]
pub struct CompletedPart {
    pub part_number: u32,
    pub etag: String,
}