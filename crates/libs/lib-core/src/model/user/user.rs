// region: ---- Imports

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::base::{self, DbBmc, prep_fields_for_update};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::post::PostProfileItem;
use crate::model::user::UserProfileStats;
use crate::model::{Error, Result};
use chrono::{DateTime, Utc};
use modql::field::{Fields, HasSeaFields, SeaField, SeaFields};
use modql::filter::{FilterNodes, ListOptions, OpValsString, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use uuid::Uuid;

// endregion: ---- Imports

// region:    --- User Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "user_typ")]
pub enum UserTyp {
    Sys,
    User,
}

// Covert custom UserTyp into sea_query::Value
impl From<UserTyp> for sea_query::Value {
    fn from(val: UserTyp) -> Self {
        val.to_string().into()
    }
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub typ: UserTyp,
    pub email_verified: bool,
    #[serde(rename = "avatar_url")]
    pub avatar_object_key: Option<String>,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct UserForPreview {
    pub id: String,
    pub username: String,
    #[serde(rename = "avatar_url")]
    pub avatar_object_key: Option<String>,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserProfileDetails {
    pub id: String,
    pub username: String,
    #[serde(rename = "avatar_url")]
    pub avatar_object_key: String,
    pub bio: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub user: UserProfileDetails,
    pub stats: UserProfileStats,
    pub posts: Vec<PostProfileItem>,

    pub is_my_profile: bool,
    pub is_following: bool,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
    pub username: String,
    pub email: String,
    pub pwd: String,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub email_verification_expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: String,
    pub username: String,
    pub email: String,

    // -- pwd and token info
    pub pwd: Option<String>, // encrypted, #_scheme_id_#....
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: String,
    pub username: String,
    pub email: String,

    // -- token info
    pub token_salt: Uuid,
}

/// Marker trait
pub trait UserBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}
impl UserBy for UserForPreview {}

// Note: Since the entity properties Iden will be given by modql
// UserIden does not have to be exhaustive, but just have the columns
#[derive(Iden)]
pub(in crate::model) enum UserPublicIden {
    #[iden = "user"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "username"]
    Username,
    #[iden = "avatar_object_key"]
    AvatarObjectKey,
}

// Note: Since the entity properties Iden will be given by modql
// UserIden does not have to be exhaustive, but just have the columns
#[derive(Iden)]
enum UserIden {
    Bio,
    Location,
    Email,
    Pwd,
    EmailVerified,
    EmailVerificationToken,
    EmailVerificationExpiresAt,
    TokenSalt,
    ResetToken,
    ResetTokenExpiresAt,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
    pub id: Option<OpValsString>,
    pub username: Option<OpValsString>,
    pub cid: Option<OpValsString>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsString>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}
// endregion: --- User Types

// region:    --- UserBmc
pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_fi: UserForInsert) -> Result<String> {
        let username = user_fi.username.clone(); // TODO: need to implement better way

        base::create::<Self, _>(ctx, mm, user_fi).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                model_error,
                Some(|table: &str, constraint: &str| {
                    if table == "user" && constraint.contains("username") {
                        Some(Error::UserAlreadyExists {
                            username: username.clone(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_preview(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<UserForPreview> {
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .expr_as(Expr::col(UserPublicIden::Id), "id")
            .expr_as(Expr::col(UserPublicIden::Username), "username")
            .expr_as(Expr::col(UserPublicIden::AvatarObjectKey), "avatar_object_key")
            .and_where(Expr::col(UserPublicIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);
        let entity = mm.dbx().fetch_one(sqlx_query).await?;
        Ok(entity)
    }

    pub async fn get_profile_details(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<UserProfileDetails> {
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .expr_as(Expr::col(UserPublicIden::Id), "id")
            .expr_as(Expr::col(UserPublicIden::Username), "username")
            .expr_as(Expr::col(UserPublicIden::AvatarObjectKey), "avatar_object_key")
            .expr_as(Expr::col(UserIden::Bio), "bio")
            .expr_as(Expr::col(UserIden::Location), "location")
            .and_where(Expr::col(UserPublicIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserProfileDetails, _>(&sql, values);

        let entity = mm.dbx().fetch_one(sqlx_query).await?;
        Ok(entity)
    }

    pub async fn first_by_username<E>(_ctx: &Ctx, mm: &ModelManager, username: &str) -> Result<Option<E>>
    where
        E: UserBy,
    {
        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::sea_idens())
            .and_where(Expr::col(UserPublicIden::Username).eq(username));

        // -- Execute query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let entity = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(entity)
    }

    pub async fn get_by_email<E>(_ctx: &Ctx, mm: &ModelManager, email: &str) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::sea_idens())
            .and_where(Expr::col(UserIden::Email).eq(email));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let entity = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(entity)
    }

    pub async fn set_reset_token(
        mm: &ModelManager,
        user_id: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(vec![
                (UserIden::ResetToken, Expr::value(token.to_string())),
                (UserIden::ResetTokenExpiresAt, Expr::value(expires_at)),
            ])
            .and_where(Expr::col(UserPublicIden::Id).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        Ok(())
    }

    pub async fn get_by_reset_token(
        mm: &ModelManager,
        token: &str,
    ) -> Result<Option<(String, Option<chrono::DateTime<Utc>>)>> {
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .expr_as(Expr::col(UserPublicIden::Id), "id")
            .expr_as(Expr::col(UserIden::ResetTokenExpiresAt), "expires_at")
            .and_where(Expr::col(UserIden::ResetToken).eq(token));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, (String, Option<chrono::DateTime<Utc>>), _>(&sql, values);

        let entity = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(entity)
    }

    pub async fn update_pwd_hash(ctx: &Ctx, mm: &ModelManager, user_id: &str, hashed_pwd: String) -> Result<()> {
        let mut fields = SeaFields::new(vec![SeaField::new(UserIden::Pwd, hashed_pwd)]);
        prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(fields.for_sea_update())
            .and_where(Expr::col(UserPublicIden::Id).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        Ok(())
    }

    pub async fn clear_reset_token(mm: &ModelManager, user_id: &str) -> Result<()> {
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(vec![
                (UserIden::ResetToken, Expr::value(Option::<String>::None)),
                (
                    UserIden::ResetTokenExpiresAt,
                    Expr::value(Option::<DateTime<Utc>>::None),
                ),
            ])
            .and_where(Expr::col(UserPublicIden::Id).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        Ok(())
    }

    pub async fn update_token_salt(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
        let new_salt = Uuid::new_v4();

        // -- Prep fields
        let mut fields = SeaFields::new(vec![SeaField::new(UserIden::TokenSalt, new_salt)]);
        prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

        // -- Build query
        let fields = fields.for_sea_update();
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(fields)
            .and_where(Expr::col(UserPublicIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        mm.dbx().execute(sqlx::query_with(&sql, values)).await?;

        tracing::info!("Token salt updated for user_id {}", id);
        Ok(())
    }

    pub async fn find_by_verification_token(
        mm: &ModelManager,
        token: &str,
    ) -> Result<Option<(String, Option<chrono::DateTime<Utc>>)>> {
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .expr_as(Expr::col(UserPublicIden::Id), "id")
            .expr_as(Expr::col(UserIden::EmailVerificationExpiresAt), "expires_at")
            .and_where(Expr::col(UserIden::EmailVerificationToken).eq(token));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, (String, Option<chrono::DateTime<Utc>>), _>(&sql, values);

        let row = mm.dbx().fetch_optional(sqlx_query).await?;
        Ok(row)
    }

    pub async fn mark_email_verified(mm: &ModelManager, user_id: &str) -> Result<()> {
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(vec![
                (UserIden::EmailVerified, Expr::value(true)),
                (UserIden::EmailVerificationToken, Expr::value(Option::<String>::None)),
                (
                    UserIden::EmailVerificationExpiresAt,
                    Expr::value(Option::<chrono::DateTime<Utc>>::None),
                ),
            ])
            .and_where(Expr::col(UserPublicIden::Id).eq(user_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let count = mm.dbx().execute(sqlx::query_with(&sql, values)).await?;

        // rows_affected
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "User",
                id: user_id.to_string(),
            });
        }

        Ok(())
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<Vec<UserFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<User>> {
        base::list::<Self, _, _>(ctx, mm, filter, list_options).await
    }

    pub async fn list_by_ids(_ctx: &Ctx, mm: &ModelManager, ids: &[String]) -> Result<Vec<UserForPreview>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .expr_as(Expr::col(UserPublicIden::Id), "id")
            .expr_as(Expr::col(UserPublicIden::Username), "username")
            .expr_as(Expr::col(UserPublicIden::AvatarObjectKey), "avatar_object_key")
            .and_where(Expr::col(UserPublicIden::Id).is_in(ids.iter().cloned().collect::<Vec<_>>()));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, UserForPreview, _>(&sql, values);
        let entities = mm.dbx().fetch_all(sqlx_query).await?;
        Ok(entities)
    }

    /// TODO: For User, deletion will require a soft-delete approach:
    ///       - Set `deleted: true`.
    ///       - Change `username` to "DELETED-_user_id_".
    ///       - Clear any other UUIDs or PII (Personally Identifiable Information).
    ///       - The automatically set `mid`/`mtime` will record who performed the deletion.
    ///       - It's likely necessary to record this action in a `um_change_log` (a user management change audit table).
    ///       - Remove or clean up any user-specific assets (messages, etc.).
    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// endregion: --- UserBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;
    use crate::_dev_utils;
    use lib_auth::pwd::{self, ContentToHash};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_bmc_create_user_01";
        let fx_email = "test_bmc_create_user01@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        // -- Prep password
        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;

        let token_salt = Uuid::new_v4();

        let _ = UserBmc::delete(&ctx, &mm, &fx_username).await;

        // -- Exec
        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash,
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
            },
        )
        .await?;

        // -- Check
        let user: User = UserBmc::get(&ctx, &mm, &user_id).await?;
        assert_eq!(user.username, fx_username);
        assert_eq!(user.email, fx_email);
        assert!(!user.email_verified);

        // -- Clean
        UserBmc::delete(&ctx, &mm, &user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // -- Exec
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .ok_or("Should have user 'demo1'")?;

        // -- Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_reset_token_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_bmc_create_user_01";
        let fx_email = "test_bmc_create_user01@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        // -- Prep password
        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;

        let token_salt = Uuid::new_v4();

        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash,
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
            },
        )
        .await?;

        let token = "reset_token_test";
        let expires_at = Utc::now() + chrono::Duration::minutes(10);

        UserBmc::set_reset_token(&mm, &user_id, token, expires_at).await?;

        let (found_id, found_exp) = UserBmc::get_by_reset_token(&mm, token).await?.expect("token should exist");

        assert_eq!(found_id, user_id);
        assert_eq!(found_exp.unwrap().timestamp(), expires_at.timestamp());

        UserBmc::clear_reset_token(&mm, &user_id).await?;

        let none = UserBmc::get_by_reset_token(&mm, token).await?;
        assert!(none.is_none());

        UserBmc::delete(&ctx, &mm, &user_id).await?;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_user_preview_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_user_preview";
        let fx_email = "test_user_preview@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;
        let token_salt = Uuid::new_v4();

        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash,
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
            },
        )
        .await?;

        let preview = UserBmc::get_preview(&ctx, &mm, &user_id).await?;
        assert_eq!(preview.username, fx_username);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_user_profile_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_user_profile";
        let fx_email = "test_user_profile@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;
        let token_salt = Uuid::new_v4();

        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash,
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
            },
        )
        .await?;

        let profile = UserBmc::get_profile_details(&ctx, &mm, &user_id).await?;
        assert_eq!(profile.username, fx_username);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_ids_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_users = vec![("list_id_01", "list01@gmail.com"), ("list_id_02", "list02@gmail.com")];

        let mut ids = vec![];
        for (username, email) in &fx_users {
            let pwd_salt = Uuid::new_v4();
            let pwd_hash = pwd::hash_pwd(ContentToHash {
                content: "Pwd123!".to_string(),
                salt: pwd_salt,
            })
            .await?;
            let token_salt = Uuid::new_v4();

            let id = UserBmc::create(
                &ctx,
                &mm,
                UserForInsert {
                    username: username.to_owned().to_string(),
                    email: email.to_owned().to_string(),
                    pwd: pwd_hash,
                    pwd_salt,
                    token_salt,
                    email_verified: false,
                    email_verification_token: None,
                    email_verification_expires_at: None,
                },
            )
            .await?;
            ids.push(id);
        }

        let users = UserBmc::list_by_ids(&ctx, &mm, &ids).await?;
        assert_eq!(users.len(), fx_users.len());

        for id in &ids {
            UserBmc::delete(&ctx, &mm, id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_pwd_hash() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_user_update_pwd";
        let fx_email = "test_user_update_pwd@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;
        let token_salt = Uuid::new_v4();

        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash.clone(),
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
            },
        )
        .await?;

        let new_pwd = "NewPwd123!";
        let new_hash = pwd::hash_pwd(ContentToHash {
            content: new_pwd.to_string(),
            salt: pwd_salt,
        })
        .await?;
        UserBmc::update_pwd_hash(&ctx, &mm, &user_id, new_hash.clone()).await?;

        let user: UserForLogin = UserBmc::get(&ctx, &mm, &user_id).await?;
        assert_eq!(user.pwd.unwrap(), new_hash);

        UserBmc::delete(&ctx, &mm, &user_id).await?;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_email_verification_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_username = "test_user_email_verif";
        let fx_email = "test_user_email_verif@gmail.com";
        let fx_pwd_clear = "TestPwd123!";

        let pwd_salt = Uuid::new_v4();
        let pwd_hash = pwd::hash_pwd(ContentToHash {
            content: fx_pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;
        let token_salt = Uuid::new_v4();

        let email_token = "email_verif_token";

        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForInsert {
                username: fx_username.to_string(),
                email: fx_email.to_string(),
                pwd: pwd_hash.clone(),
                pwd_salt,
                token_salt,
                email_verified: false,
                email_verification_token: Some(email_token.to_string()),
                email_verification_expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
            },
        )
        .await?;

        let found = UserBmc::find_by_verification_token(&mm, email_token).await?;
        assert!(found.is_some());

        UserBmc::mark_email_verified(&mm, &user_id).await?;

        let user: User = UserBmc::get(&ctx, &mm, &user_id).await?;
        assert!(user.email_verified);
        // assert!(user.email_verification_token.is_none());

        UserBmc::delete(&ctx, &mm, &user_id).await?;
        Ok(())
    }
}

// endregion: --- Tests
