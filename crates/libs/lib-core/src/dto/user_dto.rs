use serde::Serialize;

/// User for Post preview in feed, Post Details, Comments section etc. 
#[derive(Debug, Serialize, Clone)]
pub struct UserPreviewDto {
    pub id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
}
