// region: ---- Modules
use crate::ctx::Ctx;
use crate::model::base::{self, prep_fields_for_update, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use lib_auth::pwd::{self, ContentToHash};
use lib_tmail::email::emails_sender::{send_welcome_email, send_verification_email};
use modql::field::{Fields, HasSeaFields, SeaField, SeaFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;
// endregion: ---- Modules

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
	pub id: i64,
	pub username: String,
	pub email: String,
	pub typ: UserTyp,
	pub email_verified: bool,
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
	pub email_verified: bool,
	pub email_verification_token: Option<String>,
	pub email_verification_expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,
	pub email: String,

	// -- pwd and token info
	pub pwd: Option<String>, // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
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

// Note: Since the entity properties Iden will be given by modql
//       UserIden does not have to be exhaustive, but just have the columns
#[derive(Iden)]
enum UserIden {
	Id,
	Username,
	Pwd,
	EmailVerified,
	EmailVerificationToken,
	EmailVerificationExpiresAt,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
	pub id: Option<OpValsInt64>,

	pub username: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
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
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserForCreate,
	) -> Result<i64> {
		let UserForCreate {
			username,
			email,
			pwd_clear,
		} = user_c;

		// Create hash & salt for pwd
		let pwd_salt = Uuid::new_v4();
		let pwd_hash = pwd::hash_pwd(ContentToHash {
			content: pwd_clear.to_string(),
			salt: pwd_salt,
		})
		.await?;

		// Generate verification token and expiration
		let verification_token = uuid::Uuid::new_v4().to_string();
		let expires_at = chrono::Utc::now() + chrono::Duration::minutes(30);

		// -- Create the user row
		let user_fi = UserForInsert {
			username: username.to_string(),
			email: email.to_string(),
			pwd: pwd_hash,
			pwd_salt,
			email_verified: false,
			email_verification_token: Some(verification_token.clone()),
			email_verification_expires_at: Some(expires_at),
		};

		// -- Create new user
		let user_id = base::create::<Self, _>(ctx, &mm, user_fi).await.map_err(
			|model_error| {
				// Check if user exists
				Error::resolve_unique_violation(
					model_error,
					Some(|table: &str, constraint: &str| {
						if table == "user" && constraint.contains("username") {
							Some(Error::UserAlreadyExists { username: username.clone() })
						} else {
							None // Error::UniqueViolation will be created by resolve_unique_violation
						}
					}),
				)
			},
		)?;

		// // Try sending emails 
        if let Err(e) = send_welcome_email(&email, &username).await {
            tracing::info!("Failed to send welcome email to {}: {:?}", email, e);
        }

        if let Err(e) = send_verification_email(&email, &username, &verification_token).await {
            tracing::info!("Failed to send verification email to {}: {:?}", email, e);
        }

		Ok(user_id)
	}

	pub async fn get<E>(
		ctx: &Ctx, 
		mm: &ModelManager, 
		id: i64
	) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::sea_idens())
			.and_where(Expr::col(UserIden::Username).eq(username));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<User>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		// -- Prep password
		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		
		let pwd = pwd::hash_pwd(ContentToHash {
			content: pwd_clear.to_string(),
			salt: user.pwd_salt,
		})
		.await?;

		// -- Prep the data
		let mut fields = SeaFields::new(vec![SeaField::new(UserIden::Pwd, pwd)]);
		prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

		// -- Build query
		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(UserIden::Id).eq(id));

		// -- Exec query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	pub async fn verify_email(
		_ctx: &Ctx,
		mm: &ModelManager,
		token: &str,
	) -> Result<()> {
		// check if token is empty
		if token.trim().is_empty() {
			return Err(Error::EmailVerificationTokenInvalid);
		}

		// Find user by verification token
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(vec![UserIden::Id])
			.and_where(Expr::col(UserIden::EmailVerificationToken).eq(token))
			.and_where(
				Expr::col(UserIden::EmailVerificationExpiresAt)
					.gt(time_to_sea_value(serde_json::json!(chrono::Utc::now()))?),
			);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let row = mm.dbx().fetch_optional(sqlx_query).await?;

		let user_id = match row {
			Some((id,)) => id,
			None => return Err(Error::EmailVerificationTokenInvalid),
		};

		// Update user as verified
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(vec![
				(UserIden::EmailVerified, Expr::value(true)),
				(UserIden::EmailVerificationToken, Expr::value(Option::<String>::None)),
				(UserIden::EmailVerificationExpiresAt, Expr::value(Option::<chrono::DateTime<chrono::Utc>>::None)),
			])
			.and_where(Expr::col(UserIden::Id).eq(user_id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	/// TODO: For User, deletion will require a soft-delete approach:
	///       - Set `deleted: true`.
	///       - Change `username` to "DELETED-_user_id_".
	///       - Clear any other UUIDs or PII (Personally Identifiable Information).
	///       - The automatically set `mid`/`mtime` will record who performed the deletion.
	///       - It's likely necessary to record this action in a `um_change_log` (a user management change audit table).
	///       - Remove or clean up any user-specific assets (messages, etc.).
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
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
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "test_create_ok-user-01";
		let fx_pwd_clear = "test_create_ok pwd 01";
		let fx_email = "test_create_ok user123@gmail.com 01";

		// -- Exec
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: fx_pwd_clear.to_string(), 
				email: fx_email.to_string(),
			},
		)
		.await?;

		// -- Check
		let user: UserForLogin = UserBmc::get(&ctx, &mm, user_id).await?;
		assert_eq!(user.username, fx_username);

		// -- Clean
		UserBmc::delete(&ctx, &mm, user_id).await?;

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
}

// endregion: --- Tests