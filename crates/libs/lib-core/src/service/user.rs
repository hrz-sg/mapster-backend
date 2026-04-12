// region: --- Imports
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserBmc, UserForAuth, UserForCreate, UserForInsert, UserForLogin, UserStatsBmc};
use crate::service::error::{Error, Result};
use chrono::Utc;
use lib_auth::auth_config;
use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_tmail::email::emails_sender::{send_reset_pwd_email, send_verification_email, send_welcome_email};
use lib_tmail::tmail_config;
use tracing::debug;
use uuid::Uuid;
// endregion: --- Imports

// region: --- User Response structs
pub struct UserRefreshTokenResponse {
    pub user_id: String,
    pub username: String,
    pub token_salt: Uuid,
}

pub struct UserLoginResponse {
    pub user_id: String,
    pub username: String,
    pub token_salt: Uuid,
}
// endregion: --- User Response structs

// region: --- User Service
pub struct UserService;

impl UserService {
    pub async fn register(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<String> {
        let UserForCreate {
            username,
            email,
            pwd_clear,
        } = user_c;

        // -- Create tx manager
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Create hash & salt for pwd
        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;

        // -- Generate verification token and expiration
        let verification_token = uuid::Uuid::new_v4().to_string();
        let config = auth_config();
        let expires_at = Utc::now() + chrono::Duration::minutes(config.VERIFY_TOKEN_TTL_MIN);

        // -- Generate token salt
        let token_salt = Uuid::new_v4();

        // -- Create the user row
        let user_fi = UserForInsert {
            username: username.to_string(),
            email: email.to_string(),
            pwd: pwd_hash,
            pwd_salt,
            token_salt,
            email_verified: false,
            email_verification_token: Some(verification_token.clone()),
            email_verification_expires_at: Some(expires_at),
        };

        // -- DB insert
        let user_id = UserBmc::create(ctx, &mm_txn, user_fi).await?;

        // -- Create user_stats (idempotent)
        UserStatsBmc::create_for_user(ctx, &mm_txn, &user_id).await?;

        dbx.commit_txn().await?;

        // NOTE: In produtction it is better to use Outbox pattern and send email thrugh workers (Valkey, Redis)
        // -- Send email (do not block registration)
        tokio::spawn({
            let email = email.clone();
            let username = username.clone();
            async move {
                if let Err(e) = send_welcome_email(&email, &username).await {
                    tracing::warn!("Failed to send welcome email: {:?}", e);
                }
            }
        });

        // TODO: move to worket in prod
        tokio::spawn({
            let email = email.clone();
            let username = username.clone();
            let token = verification_token.clone();
            async move {
                if let Err(e) = send_verification_email(&email, &username, &token).await {
                    tracing::warn!("Failed to send verification email: {:?}", e);
                }
            }
        });

        Ok(user_id)
    }

    pub async fn login(_ctx: &Ctx, mm: &ModelManager, username: &str, pwd_clear: &str) -> Result<UserLoginResponse> {
        let root_ctx = Ctx::root_ctx();

        // -- Get the user.
        let user: UserForLogin = UserBmc::first_by_username(&root_ctx, mm, username)
            .await?
            .ok_or(Error::LoginUsernameNotFound)?;

        // Store the user_id BEFORE moving user
        let user_id = user.id.clone();

        // -- Validate the password.
        let Some(pwd) = user.pwd else {
            return Err(Error::LoginUserHasNoPwd);
        };

        let scheme_status = pwd::validate_pwd(
            ContentToHash {
                salt: user.pwd_salt,
                content: pwd_clear.to_string(),
            },
            pwd,
        )
        .await
        .map_err(|_| Error::LoginPwdNotMatching)?;

        // -- Update password scheme if needed
        if let SchemeStatus::Outdated = scheme_status {
            debug!("pwd encrypt scheme outdated, upgrading.");

            let new_hash = pwd::hash_pwd(ContentToHash {
                content: pwd_clear.to_string(),
                salt: user.pwd_salt,
            })
            .await?;

            // Use the stored user_id here instead of user.id
            UserBmc::update_pwd_hash(&root_ctx, &mm, &user_id, new_hash).await?;
        }

        Ok(UserLoginResponse {
            user_id: user.id,
            username: user.username,
            token_salt: user.token_salt,
        })
    }

    pub async fn verify_email(_ctx: &Ctx, mm: &ModelManager, token: &str) -> Result<()> {
        tracing::debug!("Verifying email token");

        if token.trim().is_empty() {
            return Err(Error::ValidationFailed("Empty verification token".into()));
        }

        let row = UserBmc::find_by_verification_token(mm, token).await?;

        let (user_id, expires_at) = match row {
            Some(v) => v,
            None => return Err(Error::ValidationFailed("Invalid verification token".into())),
        };

        if let Some(exp) = expires_at {
            if Utc::now() > exp {
                return Err(Error::ValidationFailed("Verification token expired".into()));
            }
        }

        UserBmc::mark_email_verified(mm, &user_id).await?;

        tracing::info!("Email verified for user {}", user_id);

        Ok(())
    }

    pub async fn validate_refresh_token(
        mm: &ModelManager,
        username: &str,
        token_salt_str: &str,
    ) -> Result<UserRefreshTokenResponse> {
        let ctx = Ctx::root_ctx();

        // -- Get user by username
        let user: UserForAuth = UserBmc::first_by_username(&ctx, mm, username)
            .await?
            .ok_or(Error::entity_not_found("User", ctx.user_id()))?;

        // -- Parse and check token salt
        let provided_salt =
            Uuid::parse_str(token_salt_str).map_err(|_| Error::validation_failed("Invalid token format"))?;

        // -- Check salt
        if provided_salt != user.token_salt {
            return Err(Error::validation_failed("Invalid token"));
        }

        Ok(UserRefreshTokenResponse {
            user_id: user.id,
            username: user.username,
            token_salt: user.token_salt,
        })
    }

    pub async fn request_password_reset(ctx: &Ctx, mm: &ModelManager, email: &str) -> Result<()> {
        tracing::debug!("Requesting password reset for email: {}", email);

        // -- Find User By Email
        let user_login: Option<UserForLogin> = UserBmc::get_by_email(ctx, mm, email).await?;

        let user = match user_login {
            Some(u) => u,
            None => {
                tracing::warn!("Password reset requested for non-existent email: {}", email);
                return Ok(());
            }
        };

        let user_id = &user.id;
        let username = &user.username;

        // -- Generate token & time expiration
        let reset_token = Uuid::new_v4().to_string();
        let config = auth_config();
        let expires_at = Utc::now() + chrono::Duration::minutes(config.RESET_TOKEN_TTL_MIN);

        // -- Save token into DB
        UserBmc::set_reset_token(mm, &user_id, &reset_token, expires_at).await?;

        // -- Send email
        let config = tmail_config();
        let reset_link = format!("{}/reset?token={}", config.PASSWORD_RESET_BASE_URL, reset_token);

        if let Err(e) = send_reset_pwd_email(email, &reset_link, &username).await {
            tracing::error!("Failed to send reset email to {}: {:?}", email, e);
        } else {
            tracing::info!("Password reset email sent to {}", email);
        }

        Ok(())
    }

    pub async fn reset_password(ctx: &Ctx, mm: &ModelManager, token: &str, new_pwd_clear: &str) -> Result<()> {
        // -- Find user by reset token
        let (user_id, expires_at) = UserBmc::get_by_reset_token(mm, token)
            .await?
            .ok_or(Error::ValidationFailed("Invalid token".into()))?;

        // -- Check expiration
        if let Some(exp) = expires_at {
            if Utc::now() > exp {
                return Err(Error::ValidationFailed("Token expired".into()));
            }
        }

        // -- Get user for salt
        let user: UserForLogin = UserBmc::get(ctx, mm, &user_id).await?;

        // -- Hash new password
        let new_hash = pwd::hash_pwd(ContentToHash {
            content: new_pwd_clear.to_string(),
            salt: user.pwd_salt,
        })
        .await?;

        // -- Update password + cleanup
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        UserBmc::update_pwd_hash(ctx, mm, &user_id, new_hash).await?;
        UserBmc::clear_reset_token(mm, &user_id).await?;
        UserBmc::update_token_salt(ctx, mm, &user_id).await?;

        dbx.commit_txn().await?;

        Ok(())
    }
}
// region: --- User Service

// region: --- User Service tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use serial_test::serial;

    type TestResult<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[serial]
    #[tokio::test]
    async fn test_register_and_login_ok() -> TestResult<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_usr_1";
        let fx_email = "test_usr_1@example.com";
        let fx_pwd_clear = "TestPwd123!";

        // -- Register
        let user_id = UserService::register(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd_clear: fx_pwd_clear.to_string(),
            },
        )
        .await?;

        // -- Check if user exists
        let created: UserForLogin = UserBmc::get(&ctx, &mm, &user_id).await?;
        assert_eq!(created.username, fx_username);
        assert_eq!(created.email, fx_email);
        assert!(created.pwd.is_some());

        // -- Get user stats
        let stats = UserStatsBmc::get_by_user_id(&ctx, &mm, &user_id).await?;
        assert_eq!(stats.posts_count, 0);
        assert_eq!(stats.followers_count, 0);
        assert_eq!(stats.following_count, 0);

        // -- Login
        let login_resp = UserService::login(&ctx, &mm, fx_username, fx_pwd_clear).await?;
        assert_eq!(login_resp.user_id, user_id);
        assert_eq!(login_resp.username, fx_username);
        assert_eq!(login_resp.token_salt, created.token_salt);

        // -- Clean
        UserBmc::delete(&ctx, &mm, &user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_register_duplicate_username_ok() -> TestResult<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_usr_1";
        let fx_email1 = "email1@example.com";
        let fx_email2 = "email2@example.com";
        let fx_pwd_clear = "Pwd123!";

        // -- Create first user
        let user_id1 = UserService::register(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                email: fx_email1.to_string(),
                pwd_clear: fx_pwd_clear.to_string(),
            },
        )
        .await?;

        // -- Try to create user with the same username
        let result = UserService::register(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                email: fx_email2.to_string(),
                pwd_clear: fx_pwd_clear.to_string(),
            },
        )
        .await;

        assert!(result.is_err());

        // -- Clean
        UserBmc::delete(&ctx, &mm, &user_id1).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_login_wrong_password_ok() -> TestResult<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "usr_test_1";
        let fx_email = "usr_test_1@example.com";
        let fx_pwd_clear = "pwd123";

        let user_id = UserService::register(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd_clear: fx_pwd_clear.to_string(),
            },
        )
        .await?;

        // -- Try loggin with different password
        let result = UserService::login(&ctx, &mm, fx_username, "pwd12345").await;
        assert!(matches!(result, Err(Error::LoginPwdNotMatching)));

        // -- Clean
        UserBmc::delete(&ctx, &mm, &user_id).await?;

        Ok(())
    }
}
// endregion: --- User Service tests
