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
pub(in crate::email) async fn send_email_with_template(
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
// endregion: ---- Send Email

// region: ---- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_processing_ok() {
        let fx_template = "Hello {{name}}, your code is {{code}}";
        let fx_placeholders = vec![
            ("{{name}}".to_string(), "Alice".to_string()),
            ("{{code}}".to_string(), "123456".to_string()),
        ];
        
        let mut result = fx_template.to_string();
        for (k, v) in &fx_placeholders {
            result = result.replace(k, v);
        }
        
        assert_eq!(result, "Hello Alice, your code is 123456");
    }

    // Validate emails
    #[test]
    fn test_email_validation_ok() {
        // It wokrs
        let fx_valid_email = "test@example.com".parse::<lettre::Address>();
        assert!(fx_valid_email.is_ok());
    }

    // Validate emails
    #[test]
    fn test_email_validation_err() {
        // Fails
        let fx_invalid_email = "not-an-email".parse::<lettre::Address>();
        assert!(fx_invalid_email.is_err());
    }

    // Test creating email message
    #[test]
    fn test_email_message_building() {
        let fx_from_email = "from@example.com".parse().unwrap();
        let fx_to_email = "to@example.com".parse().unwrap();
        let fx_subject = "Test Subject";
        let fx_html_content = "<h1>Test</h1>";
        
        let email = Message::builder()
            .from(fx_from_email)
            .to(fx_to_email)
            .subject(fx_subject)
            .header(header::ContentType::TEXT_HTML)
            .singlepart(SinglePart::html(fx_html_content.to_string()));
            
        assert!(email.is_ok());
    }
}
// endregion: ---- Tests