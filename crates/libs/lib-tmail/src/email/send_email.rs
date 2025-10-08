// region: ---- Modules
use lettre::{
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport,
    Transport
};
use crate::tmail_config;
use super::error::{Error, Result};
// endregion: ---- Modules

// region: ---- Send Email
pub async fn send_email_with_template(
    to_email: &str,
    subject: &str,
    template: &str,
    placeholders: &[(String, String)]
) -> Result<()> {
    let config = tmail_config();
    let smtp_username = &config.SMTP_USERNAME;
    let smtp_pwd = &config.SMTP_PWD;
    let smtp_server = &config.SMTP_SERVER;
    let smtp_port = config.SMTP_PORT;

    // Process template with placeholders
    let mut html_template = template.to_string();
    for (k, v) in placeholders {
        html_template = html_template.replace(k, v);
    }

    let email = Message::builder()
        .from(smtp_username.parse().map_err(|_| Error::InvalidEmail)?)
        .to(to_email.parse().map_err(|_| Error::InvalidEmail)?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(SinglePart::html(html_template))
        .map_err(|_| Error::TemplateProcessing)?;

    let creds = Credentials::new(smtp_username.clone(), smtp_pwd.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)
            .map_err(|_| Error::SmtpConfig)?
            .credentials(creds)
            .port(smtp_port)
            .build();

    // Send email and return result
    mailer.send(&email).map_err(|_| Error::SendFailed)?;
    
    Ok(())
}

// Legacy function for backward compatibility
pub async fn send_email(
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)]
) -> Result<()> {
    use tokio::fs;
    
    let template = fs::read_to_string(template_path).await
        .map_err(|_| Error::TemplateNotFound)?;
    send_email_with_template(to_email, subject, &template, placeholders).await
}
// endregion: ---- Send Email