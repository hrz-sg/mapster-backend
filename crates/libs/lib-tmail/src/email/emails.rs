use super::send_email::send_email_with_template;
use super::error::Result;
use crate::tmail_config;

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

fn create_verification_link(base_url: &str, token: &str) -> String {
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
