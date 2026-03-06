use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc, prep_fields_for_create, prep_fields_for_update};
use crate::model::{Error, ModelManager, Result};
use modql::field::{Fields, HasSeaFields};
use modql::filter::{FilterNodes, ListOptions, OpValsString};
use sea_query::{Expr, Iden, OnConflict, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- JourneyPost Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct JourneyPost {
    pub journey_id: String,
    pub post_id: String,
    pub sort_order: i32,
}

#[derive(Fields, Deserialize)]
pub struct JourneyPostForCreate {
    pub journey_id: String,
    pub post_id: String,
    pub sort_order: i32,
}

#[derive(Fields, Default, Deserialize)]
pub struct JourneyPostForUpdate {
    pub post_id: String,
    pub sort_order: i32,
}

#[derive(Fields, Default, Deserialize)]
pub struct AddPostToJourney {
    pub post_id: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct JourneyPostFilter {
    pub journey_id: Option<OpValsString>,
    pub post_id: Option<OpValsString>,
}

// endregion: ---- JourneyPost Types

// region: ---- JourneyPostIden
#[derive(Iden, Clone)]
pub enum JourneyPostIden {
    #[iden = "journey_post"]
    Table,
    #[iden = "journey_id"]
    JourneyId,
    #[iden = "post_id"]
    PostId,
    #[iden = "sort_order"]
    SortOrder,
}
// endregion: ---- JourneyPostIden

// region: ---- JourneyPostBmc
pub struct JourneyPostBmc;

impl DbBmc for JourneyPostBmc {
    const TABLE: &'static str = "journey_post";

    fn has_id() -> bool {
        false
    }

    fn timestamp_fields() -> base::TimestampType {
        base::TimestampType::CtimeMtime
    }
}

impl JourneyPostBmc {
    pub async fn find_journey_id_by_post(ctx: &Ctx, mm: &ModelManager, post_id: &str) -> Result<Option<String>> {
        let filter = vec![JourneyPostFilter {
            post_id: Some(post_id.into()),
            ..Default::default()
        }];

        Ok(base::first::<Self, JourneyPost, _>(ctx, mm, Some(filter), None)
            .await?
            .map(|jp| jp.journey_id))
    }

    pub async fn exists(ctx: &Ctx, mm: &ModelManager, journey_id: &str, post_id: &str) -> Result<bool> {
        let filter = vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            post_id: Some(post_id.into()),
        }];

        base::exists::<Self, _>(ctx, mm, filter).await
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, journey_post_c: JourneyPostForCreate) -> Result<()> {
        // -- Extract fields (as in base::create)
        let mut fields = journey_post_c.not_none_sea_fields();
        prep_fields_for_create::<Self>(&mut fields, ctx.user_id());

        // -- Build query
        let (columns, sea_values) = fields.for_sea_insert();
        let mut query = Query::insert();
        query
            .into_table(Self::table_ref())
            .columns(columns)
            .values(sea_values)?
            .on_conflict(
                OnConflict::columns([JourneyPostIden::JourneyId, JourneyPostIden::PostId])
                    .do_nothing()
                    .to_owned(),
            );

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, journey_id: &str, post_id: &str) -> Result<JourneyPost> {
        let filter = vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            post_id: Some(post_id.into()),
        }];

        base::first_by_composite_key::<Self, JourneyPost, _>(ctx, mm, Some(filter))
            .await?
            .ok_or_else(|| Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{journey_id}:{post_id}"),
            })
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<JourneyPostFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<JourneyPost>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update_post_position(
        _ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        post_u: JourneyPostForUpdate,
    ) -> Result<()> {

        let JourneyPostForUpdate {
            post_id,
            sort_order,
        } = post_u;

        let mut query = Query::update();
        query
            .table(JourneyPostIden::Table)
            .value(JourneyPostIden::SortOrder, sort_order)
            .and_where(Expr::col(JourneyPostIden::JourneyId).eq(journey_id))
            .and_where(Expr::col(JourneyPostIden::PostId).eq(post_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        mm.dbx().execute(sqlx_query).await?;
        
        Ok(())
    }

    // --- Update sort order for journey post
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        journey_post_u: JourneyPostForUpdate,
    ) -> Result<()> {
        let post_id = journey_post_u.post_id.clone();
        // -- Extract fields (as in base::create)
        let mut fields = journey_post_u.not_none_sea_fields();
        prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

        // -- Build query
        let fields = fields.for_sea_update();
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(fields)
            .and_where(Expr::col(JourneyPostIden::JourneyId).eq(journey_id))
            .and_where(Expr::col(JourneyPostIden::PostId).eq(post_id.clone()));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{journey_id}:{post_id}"),
            })
        } else {
            Ok(())
        }
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, journey_id: &str, post_id: &str) -> Result<()> {
        let mut query = Query::delete();
        query
            .from_table(Self::table_ref())
            .and_where(Expr::col(JourneyPostIden::JourneyId).eq(journey_id))
            .and_where(Expr::col(JourneyPostIden::PostId).eq(post_id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let count = mm.dbx().execute(sqlx_query).await?;

        if count == 0 {
            Err(Error::EntityNotFound {
                entity: Self::TABLE,
                id: format!("{journey_id}:{post_id}"),
            })
        } else {
            Ok(())
        }
    }
}

// endregion: ---- JourneyPostBmc
