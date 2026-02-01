// region: -- Module
use crate::error::Result;
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use lib_core::{
    ctx::Ctx,
    model::{
        ModelManager,
        journey::{Journey, JourneyForUpdate},
        post::PostWithUser,
    },
    service::journey::JourneyService,
};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use tracing::debug;
// endregion: -- Module

// region: -- Journey handlers
pub async fn api_create_journey_handler(
    State(mm): State<ModelManager>,
    mut multipart: Multipart,
) -> Result<Json<CreateJourneyResponse>> {
    debug!("{:<12} - api_create_journey_handler", "HANDLER");

    let ctx = Ctx::root_ctx();

    let mut title = String::new();
    let mut description = String::new();
    let mut cover_url: Option<String> = None;
    let mut post_ids: Vec<String> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "title" => title = field.text().await.unwrap_or_default(),
            "description" => description = field.text().await.unwrap_or_default(),
            "cover_url" => cover_url = Some(field.text().await.unwrap_or_default()),
            "post_ids[]" => {
                let id = field.text().await.unwrap_or_default();
                if !id.is_empty() {
                    post_ids.push(id);
                }
            }
            _ => (),
        }
    }

    let input = CreateJourneyRequest {
        title,
        description,
        cover_url,
        post_ids,
    };

    input.validate()?;

    let journey = JourneyService::create_from_posts(
        &ctx,
        &mm,
        &input.title.clone(),
        &input.description.clone(),
        input.cover_url.as_deref(),
        input.post_ids.iter().map(|s| s.as_str()).collect(),
    )
    .await?;

    Ok(Json(CreateJourneyResponse {
        success: true,
        journey_id: Some(journey.id),
        message: "Journey created successfully!".into(),
    }))
}

pub async fn api_get_journey_handler(
    State(mm): State<ModelManager>,
    Path(journey_id): Path<String>,
) -> Result<Json<GetJourneyResponse>> {
    let ctx = Ctx::root_ctx();

    let journey = JourneyService::get_with_posts(&ctx, &mm, &journey_id).await?;

    Ok(Json(GetJourneyResponse { success: true, journey }))
}

pub async fn api_update_journey_handler(
    State(mm): State<ModelManager>,
    Path(journey_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<UpdateJourneyResponse>> {
    let ctx = Ctx::root_ctx();

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut cover_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "title" => title = Some(field.text().await.unwrap_or_default()),
            "description" => description = Some(field.text().await.unwrap_or_default()),
            "cover_url" => cover_url = Some(field.text().await.unwrap_or_default()),
            _ => (),
        }
    }

    let journey_update = JourneyForUpdate {
        title,
        description,
        cover_media_url: cover_url,
        is_published: None,
    };

    let updated_journey = JourneyService::update(&ctx, &mm, &journey_id, journey_update).await?;

    Ok(Json(UpdateJourneyResponse {
        success: true,
        journey: updated_journey,
        message: "Journey updated successfully!".into(),
    }))
}

pub async fn api_delete_journey_handler(
    State(mm): State<ModelManager>,
    Path(journey_id): Path<String>,
) -> Result<Json<DeleteJourneyResponse>> {
    let ctx = Ctx::root_ctx();

    JourneyService::delete(&ctx, &mm, &journey_id).await?;

    Ok(Json(DeleteJourneyResponse {
        success: true,
        message: "Journey deleted successfully!".into(),
    }))
}

pub async fn api_save_journey_handler(
    State(mm): State<ModelManager>,
    Path(journey_id): Path<String>,
) -> Result<Json<GenericResponse>> {
    let ctx = Ctx::root_ctx();
    JourneyService::save_journey(&ctx, &mm, &journey_id).await?;
    Ok(Json(GenericResponse {
        success: true,
        message: "Journey saved successfully".into(),
    }))
}

pub async fn api_unsave_journey_handler(
    State(mm): State<ModelManager>,
    Path(journey_id): Path<String>,
) -> Result<Json<GenericResponse>> {
    let ctx = Ctx::root_ctx();
    JourneyService::unsave_journey(&ctx, &mm, &journey_id).await?;
    Ok(Json(GenericResponse {
        success: true,
        message: "Journey unsaved successfully".into(),
    }))
}
// endregion: -- Journey handlers

// region: -- Journey handlers request & response structs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateJourneyRequest {
    #[validate(min_items = 2, message = "Journey must have at least 2 posts")]
    pub post_ids: Vec<String>,

    #[validate(min_length = 1, message = "Title cannot be empty")]
    pub title: String,

    pub description: String,
    pub cover_url: Option<String>,
}

// --- Response structs
#[derive(Debug, Serialize)]
pub struct CreateJourneyResponse {
    pub success: bool,
    pub journey_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GetJourneyResponse {
    pub success: bool,
    pub journey: (Journey, Vec<PostWithUser>),
}

#[derive(Debug, Serialize)]
pub struct UpdateJourneyResponse {
    pub success: bool,
    pub journey: Journey,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteJourneyResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
}
// endregion: -- Journey handlers request & response structs
