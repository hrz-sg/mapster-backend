use crate::{ctx::Ctx, model::{ModelManager, journey_save::{JourneySaveBmc, JourneySaveFilter, JourneySaveForCreate}}};
use crate::service::error::Result;

pub struct JourneySaveService;

impl JourneySaveService {
    /// --- Check if the user saved the journey
    pub async fn has_current_user_saved(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<bool> {

        let result = JourneySaveBmc::exists(ctx, mm, journey_id, ctx.user_id()).await?;

        Ok(result)
    }
        
    /// --- Count all saves for the journey (for public statistics)
    pub async fn count_by_journey(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<i64> {

        let filters = vec![JourneySaveFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }];
        
        let count = JourneySaveBmc::count(ctx, mm, Some(filters)).await?;

        Ok(count)
    }
}
