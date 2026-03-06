use std::collections::HashMap;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::store::oss::{CompletedPart, build_object_key};
use crate::service::error::{Result, Error};
use lib_utils::file::{MULTIPART_THRESHOLD, calc_part_size, validate_file_meta};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MediaUploadInfo {
    pub file_id: String,
    pub object_key: String,
    pub file_name: String,
    pub media_type: String,
    pub multipart: bool,
    pub upload_id: Option<String>,
    pub urls: Vec<String>,
    pub part_size: Option<u64>,
    pub total_parts: Option<u32>,
}

pub struct UploadMediaService;

impl UploadMediaService {
    // Client Requests Upload Initialization → Backend: POST /uploads/initialize
    pub async fn init_upload_session(
        _ctx: &Ctx,
        mm: &ModelManager,
        payload: InitUploadPayload,
    ) -> Result<InitUploadSessionResp> {

        let InitUploadPayload {
            entity,
            entity_id,
            files,
        } = payload;

        let session_id = Uuid::new_v4().to_string();

        let mut media_infos = Vec::new();

        for file in files {

            // Validate file - extension, size (1GB video, 100MB image), media type 
            let media_type = validate_file_meta(&file.filename, &file.content_type, file.size)?;
            
            let object_key= build_object_key(&entity, &entity_id, &file.filename);

            // decide if multipart
            let is_multipart = file.size > MULTIPART_THRESHOLD; // 10MB threshold

            if is_multipart {
                
                let part_size = calc_part_size(file.size);
                let part_count = ((file.size + part_size - 1) / part_size) as u32;

                // Generate presigned URL for each part
                let (upload_id, urls) = mm.bucket().presigned_url_multipart(
                    &object_key,
                    &file.filename,
                    part_count,
                    3600,
                ).await?;

                media_infos.push(MediaUploadInfo {
                    file_id: file.file_id,
                    object_key: object_key.clone(),
                    file_name: file.filename,
                    media_type,
                    multipart: true,
                    upload_id: Some(upload_id),
                    urls: urls.into_iter().map(|u| u.url).collect(),
                    part_size: Some(part_size),
                    total_parts: Some(part_count),
                });
            } else {
                let url = &mm.bucket().presigned_url(
                    &object_key,
                    &file.content_type,
                );

                media_infos.push(MediaUploadInfo {
                    file_id: file.file_id,
                    object_key: object_key.clone(),
                    file_name: file.filename,
                    media_type,
                    multipart: false,
                    upload_id: None,
                    urls: vec![url.url.clone()],
                    part_size: None,
                    total_parts: None,
                });
            }
        }

        Ok(InitUploadSessionResp {
            session_id,
            uploads: media_infos,
        })
    }

    // Client can now request presigned URLs for per part → Backend: POST /uploads/presigned-url
    // Note: Client does retries (MAX_RETRIES = 3)
    pub async fn generate_presigned_url_for_part(
        _ctx: &Ctx,
        mm: &ModelManager,
        payload: UploadPartPayload,
    ) -> Result<PartPresignedUrl> {
        if payload.part_number == 0 || payload.part_number > 10_000 {
            return Err(Error::validation_failed("Invalid part number"));
        }

        // --- Generate presigned URL for this part
        let signed = if payload.upload_id.is_empty() {
            // single file
            mm.bucket().presigned_url(
                    &payload.object_key,
                    &payload.content_type,
                )
        } else {
            // multipart
            mm.bucket().presigned_url_for_part(
                &payload.object_key,
                &payload.upload_id,
                payload.part_number,
                &payload.content_type,
            )
        };

        Ok(PartPresignedUrl {
            part_number: payload.part_number,
            url: signed.url,
            headers: signed.headers
        })
    }

    // UI calls Backend to complete the upload → Backend: POST /uploads/complete
    pub async fn complete_upload_session(
        _ctx: &Ctx,
        mm: &ModelManager,
        payload: CompleteUploadPayload,
    ) -> Result<CompleteUploadResp> {
        // Validate parts from upload

        let mut parts = payload.parts;
        parts.sort_by_key(|p|p.part_number);

        let mut last = 0;
        for p in &parts {
            if p.part_number == 0 || p.part_number <= last {
                return Err(Error::validation_failed("Invalid part order"));
            }
            if p.part_number != last + 1 {
                return Err(Error::validation_failed("Missing part"));
            }
            last = p.part_number
        }

        let etag = mm.bucket()
            .complete_multipart_upload(
                &payload.object_key,
                &payload.upload_id,
                parts,
            )
            .await?;

        info!("Completing multipart upload: object_key={}, upload_id={}", payload.object_key, payload.upload_id);

        Ok(CompleteUploadResp {
            object_key: payload.object_key,
            etag,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PresignedPart {
    pub part_number: u32,
    pub url: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UploadFile {
    pub file_id: String,
    pub path: String, // local path
    pub parts: Vec<PresignedPart>,
}

#[derive(Serialize, Deserialize)]
pub struct InitUploadPayload {
    entity: String,
    entity_id: String,
    files: Vec<InitFilePayload>,
}

#[derive(Serialize, Deserialize)]
pub struct InitFilePayload {
    file_id: String,
    filename: String,
    size: u64,
    content_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct InitUploadSessionResp {
    session_id: String,
    uploads: Vec<MediaUploadInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadPartPayload {
    pub object_key: String,
    pub upload_id: String,
    pub part_number: u32,
    pub content_type: String, // mime
}

#[derive(Serialize, Deserialize)]
pub struct PartPresignedUrl {
    pub part_number: u32,
    pub url: String,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteUploadPayload {
    pub upload_id: String,
    pub object_key: String,
    pub parts: Vec<CompletedPart>,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteUploadResp {
    pub object_key: String,
    pub etag: String,
}