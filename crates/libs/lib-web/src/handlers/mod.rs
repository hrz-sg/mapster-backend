// --- Auth
pub mod handlers_register;
pub mod handlers_email;
pub mod handlers_login;

// --- Posts & Posts related handlers
pub mod handlers_post;
pub mod handlers_post_likes;
pub mod handlers_post_comments;

// --- Users
pub mod handlers_user_profile;
pub mod handlers_user_follow;

// --- Middleware
pub mod handlers_tokens;
pub mod mw_req_stamp;
