use lib_utils::envs::{get_env_parse, get_env};
use std::sync::OnceLock;

pub fn tmail_config() -> &'static EmailConfig {
    static INSTANCE: OnceLock<EmailConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        EmailConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING EMAIL CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub SMTP_USERNAME: String,
    pub SMTP_PWD: String,
    pub SMTP_SERVER: String,
    pub SMTP_PORT: u16,
    pub USE_TLS: bool,
    pub EMAIL_VERIFICATION_BASE_URL: String,
    pub SUPPORT_EMAIL: String,
}

impl EmailConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(Self {
            SMTP_USERNAME: get_env("SMTP_USERNAME")?,
            SMTP_PWD: get_env("SMTP_PWD")?,
            SMTP_SERVER: get_env("SMTP_SERVER")?,
            SMTP_PORT: get_env_parse("SMTP_PORT")?,
            USE_TLS: get_env_parse("SMTP_USE_TLS")?,
            EMAIL_VERIFICATION_BASE_URL: get_env_parse("EMAIL_VERIFICATION_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8000/api/auth/verify".to_string()),
            SUPPORT_EMAIL: std::env::var("SUPPORT_EMAIL")
                .unwrap_or_else(|_| "support@mapster.com".to_string()),
        })
    }
}

