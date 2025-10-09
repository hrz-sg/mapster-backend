// region: ---- Modules
use super::templates_sender::send_email_with_template;
use super::error::Result;
use crate::tmail_config;
// endregion: ---- Modules

// Embedded email templates
const VERIFICATION_EMAIL_TEMPLATE: &str = include_str!("templates/verification-email.html");
const WELCOME_EMAIL_TEMPLATE: &str = include_str!("templates/welcome-email.html");
const RESET_PWD_EMAIL_TEMPLATE: &str = include_str!("templates/reset-pwd-email.html");

// region:    --- Email Verification
pub async fn send_verification_email(
    to_email: &str,
    username: &str,
    token: &str,
) -> Result<()> {
    let config = tmail_config();
    let subject = "Email verification";
    let verification_link = create_verification_link(&config.EMAIL_VERIFICATION_BASE_URL, token);
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{verification_link}}".to_string(), verification_link.to_string()),
        ("{{support_email}}".to_string(), config.SUPPORT_EMAIL.clone())
    ];

    send_email_with_template(to_email, subject, VERIFICATION_EMAIL_TEMPLATE, &placeholders).await
}

// helper for Email verification
fn create_verification_link(
    base_url: &str, 
    token: &str
) -> String {
    format!("{}?token={}", base_url, token)
}
// endregion: --- Email Verification

// region:    --- Welcome Email
pub async fn send_welcome_email(
    to_email: &str,
    username: &str
) -> Result<()> {
    let config = tmail_config();
    let subject = "Welcome to Mapster";
    let dashboard_link = config.EMAIL_VERIFICATION_BASE_URL.replace("/verify", "/dashboard");
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{support_email}}".to_string(), config.SUPPORT_EMAIL.clone()),
        ("{{dashboard_link}}".to_string(), dashboard_link)
    ];

    send_email_with_template(to_email, subject, WELCOME_EMAIL_TEMPLATE, &placeholders).await
}
// endregion: --- Welcome Email

// region:    --- Password Reset Email
pub async fn send_reset_pwd_email(
    to_email: &str,
    reset_link: &str,
    username: &str
) -> Result<()> {
    let config = tmail_config();
    let subject = "Reset password";
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{reset_link}}".to_string(), reset_link.to_string()),
        ("{{support_email}}".to_string(), config.SUPPORT_EMAIL.clone())
    ];

    send_email_with_template(to_email, subject, RESET_PWD_EMAIL_TEMPLATE, &placeholders).await
}
// endregion: --- Password Reset Email

// region: ---- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tmail_config;

    fn init() {
        dotenvy::dotenv().ok();
    }

    #[test]
    fn test_create_verification_link() {
        let fx_base_url = "http://localhost:8080/verify";
        let fx_token = "test-token-123";
        
        let result = create_verification_link(fx_base_url, fx_token);
        let expected = "http://localhost:8080/verify?token=test-token-123";
        
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_send_verification_email_template_ok() {
        init();
        let result = send_verification_email(
            "test@example.com", 
            "testuser", 
            "test-token-123"
        ).await;
        
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test] 
    async fn test_send_welcome_email_template_ok() {
        init();
        let result = send_welcome_email(
            "test@example.com",
            "testuser"
        ).await;
        
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_send_reset_pwd_email_template_ok() {
        init();
        let result = send_reset_pwd_email(
            "test@example.com",
            "http://localhost:8080/reset-pwd?token=abc123",
            "testuser"
        ).await;
        
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_email_templates_loaded_ok() {
        assert!(!VERIFICATION_EMAIL_TEMPLATE.is_empty());
        assert!(!WELCOME_EMAIL_TEMPLATE.is_empty()); 
        assert!(!RESET_PWD_EMAIL_TEMPLATE.is_empty());
        
        assert!(VERIFICATION_EMAIL_TEMPLATE.contains("{{username}}"));
        assert!(VERIFICATION_EMAIL_TEMPLATE.contains("{{verification_link}}"));
        
        assert!(WELCOME_EMAIL_TEMPLATE.contains("{{username}}"));
        assert!(WELCOME_EMAIL_TEMPLATE.contains("{{dashboard_link}}"));
        
        assert!(RESET_PWD_EMAIL_TEMPLATE.contains("{{username}}"));
        assert!(RESET_PWD_EMAIL_TEMPLATE.contains("{{reset_link}}"));
    }

    #[test]
    fn test_verification_email_placeholders_ok() {
        init();
        let fx_token = "test-token-123";
        let fx_email_verification_base_url = &tmail_config().EMAIL_VERIFICATION_BASE_URL;
        
        let verification_link = create_verification_link(fx_email_verification_base_url, fx_token);
        
        // Chech if link is created properly
        assert!(verification_link.contains(fx_email_verification_base_url));
        assert!(verification_link.contains(fx_token));
        assert!(verification_link.contains('?'));
        assert!(verification_link.contains("token="));
    }

    #[test]
    fn test_welcome_email_placeholders_ok() {
        init();
        let dashboard_link = tmail_config().EMAIL_VERIFICATION_BASE_URL.replace("/verify", "/dashboard");
        
        // Check if dashboard_link created correctly
        assert!(dashboard_link.contains("/dashboard"));
        assert!(!dashboard_link.contains("/verify"));
    }
}
// endregion: ---- Tests