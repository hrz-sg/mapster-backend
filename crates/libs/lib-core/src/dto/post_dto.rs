use serde::Serialize;
use crate::{dto::user_dto::UserPreviewDto, model::post_media::PostMedia};

/// DTO Post feed item
#[derive(Debug, Serialize)]
pub struct PostFeedItemDto {
    pub id: i64,
    pub title: String,
    pub author: UserPreviewDto,
    pub thumbnail_url: Option<String>,
    pub media_count: i32,
    pub has_video: bool,
    pub like_count: i64,
}

/// DTO Post Details
#[derive(Debug, Serialize)]
pub struct PostDetailDto {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub author: UserPreviewDto,
    pub thumbnail_url: Option<String>,
    pub medias: Vec<PostMedia>,
    pub like_count: i64,
    pub comment_count: i64,
    pub saved_count: i64,
}
