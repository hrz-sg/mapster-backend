pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // SMTP Configuration
    SmtpConfig,
    SmtpConnection,
    SmtpAuth,
    
    // Email Content
    InvalidEmail,
    InvalidSubject,
    TemplateNotFound,
    TemplateProcessing,
    
    // Sending
    SendFailed,
    RecipientRejected,
    ServerUnavailable,
    
    // Configuration
    MissingConfig(String),
    InvalidConfig(String),
}

// region: --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region: --- Client Error
impl Error {
    pub fn client_message(&self) -> &'static str {
        match self {
            Error::SmtpConfig | Error::SmtpConnection | Error::SmtpAuth => {
                "Email service temporarily unavailable"
            }
            Error::InvalidEmail => "Invalid email address format",
            Error::InvalidSubject => "Invalid email subject",
            Error::TemplateNotFound | Error::TemplateProcessing => {
                "Email template error"
            }
            Error::SendFailed | Error::RecipientRejected | Error::ServerUnavailable => {
                "Failed to send email"
            }
            Error::MissingConfig(_) | Error::InvalidConfig(_) => {
                "Email service configuration error"
            }
        }
    }
}
// endregion: --- Client Error
