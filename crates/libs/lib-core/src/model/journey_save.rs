use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, TimestampType};
use crate::model::{Error, ModelManager, Result};
use modql::field::Fields;
use modql::filter::{FilterNodes, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- JourneySave Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct JourneySave {
    pub journey_id: String,
    pub user_id: String,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Deserialize)]
pub struct JourneySaveForCreate {
    pub journey_id: String,
    pub user_id: String,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct JourneySaveFilter {
    pub journey_id: Option<OpValsString>,
    pub user_id: Option<OpValsString>,
}
// endregion: ---- JourneySave Types

// region: ---- JourneySaveIden
#[derive(Iden)]
pub enum JourneySaveIden {
    Table,
    JourneyId,
    UserId,
    Ctime,
}
// endregion: ---- JourneySaveIden

// region: ---- JourneySaveBmc
pub struct JourneySaveBmc;

impl DbBmc for JourneySaveBmc {
    const TABLE: &'static str = "journey_save";

    fn has_id() -> bool { 
        false 
    }

    fn has_user_id() -> bool {
        true
    }

    fn timestamp_fields() -> base::TimestampType {
        TimestampType::CtimeOnly
    }
}

impl JourneySaveBmc {
    pub async fn exists(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: &str, 
        user_id: &str
    ) -> Result<bool> {
        let filter = vec![JourneySaveFilter {
            journey_id: Some(journey_id.into()),
            user_id: Some(user_id.into()),
        }];
        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, journey_save_c: JourneySaveForCreate) -> Result<()> {
        base::create_without_id::<Self, _>(ctx, mm, journey_save_c).await
    }
    
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        
        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(JourneySaveIden::JourneyId).eq(journey_id))
            .and_where(Expr::col(JourneySaveIden::UserId).eq(ctx.user_id()));
        
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
        filter: Option<Vec<JourneySaveFilter>>
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filter).await
    }
}
// endregion: ---- JourneySaveBmc