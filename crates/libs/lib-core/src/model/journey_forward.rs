use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- JourneyForward Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct JourneyForward {
    pub journey_id: String,
    pub user_id: String,
    pub forward_to_user_id: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct JourneyForwardForCreate {
    pub journey_id: String,
    pub user_id: String,
    pub forward_to_user_id: String,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct JourneyForwardFilter {
    pub journey_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
}
// endregion: ---- JourneyForward Types

// region: ---- JourneyForwardIden
#[derive(Iden)]
pub enum JourneyForwardIden {
    Table,
    JourneyId,
    UserId,
    ForwardToUserId,
    Ctime,
}
// endregion: ---- JourneyForwardIden

// region: ---- JourneyForwardBmc
pub struct JourneyForwardBmc;

impl DbBmc for JourneyForwardBmc {
    const TABLE: &'static str = "journey_forward";
    
    fn has_id() -> bool { 
        false 
    }

    fn has_user_id() -> bool {
        true
    }

    fn timestamp_fields() -> TimestampType {
        TimestampType::CtimeOnly
    }
}

impl JourneyForwardBmc {
    pub async fn exists(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: &str, 
        user_id: &str
    ) -> Result<bool> {
        let filter = vec![JourneyForwardFilter {
            journey_id: Some(journey_id.into()),
            user_id: Some(user_id.into())
        }];
        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_forward_c: JourneyForwardForCreate,
    ) -> Result<()> {
        
        base::create_without_id::<Self, _>(ctx, mm, journey_forward_c).await
    }

    // --- Delete for current user (ctx.user_id())
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(JourneyForwardIden::JourneyId).eq(journey_id))
            .and_where(Expr::col(JourneyForwardIden::UserId).eq(ctx.user_id()));
        
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;
            
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{journey_id}:{}", ctx.user_id()),
            });
        }
        
        Ok(())
    }
    
    pub async fn count(
        ctx: &Ctx,  
        mm: &ModelManager, 
        filter: Option<Vec<JourneyForwardFilter>>
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }
    
    pub async fn list(
        ctx: &Ctx, 
        mm: &ModelManager, 
        filter: Option<Vec<JourneyForwardFilter>>, 
        list_options: Option<ListOptions>
    ) -> Result<Vec<JourneyForward>> {
        base::list::<Self, JourneyForward, _>(ctx, mm, filter, list_options).await
    }
}
// endregion: ---- JourneyForwardBmc