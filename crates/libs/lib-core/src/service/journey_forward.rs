use crate::{
    ctx::Ctx, 
    model::{ModelManager,journey_forward::{JourneyForwardBmc, JourneyForwardFilter, JourneyForwardForCreate}}
};
use crate::service::error::Result;

pub struct JourneyForwardService;

impl JourneyForwardService {
    /// --- Save joiurney for current user
    pub async fn forward_for_current_user(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: &str, 
        forward_to_user_id: &str
    ) -> Result<()> {
        // -- Check if user already forwarded
        if JourneyForwardBmc::exists(ctx, mm, journey_id, ctx.user_id()).await? {
            return Ok(()); // if forwarded, do nothing
        }

        let journey_forward_c = JourneyForwardForCreate {
            journey_id: journey_id.to_string(),
            user_id: ctx.user_id().to_string(),
            forward_to_user_id: forward_to_user_id.to_string(),
        };

        let result = JourneyForwardBmc::create(ctx, mm, journey_forward_c).await?;

        Ok(result)
    }
    
    /// Delete saved content for ctx user
    pub async fn unforward_for_current_user(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: &str
    ) -> Result<()> {
        
        let result = JourneyForwardBmc::delete(ctx, mm, journey_id).await?;

        Ok(result)
    }

    /// --- Check if the user forwarded the journey
    pub async fn has_current_user_forwarded(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: &str
    ) -> Result<bool> {

        let result = JourneyForwardBmc::exists(ctx, mm, journey_id, ctx.user_id()).await?;

        Ok(result)
    }
        
    /// --- Count all forwards for the journey
    pub async fn count_by_journey(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<i64> {

        let filter = vec![JourneyForwardFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }];
        
        let count = JourneyForwardBmc::count(ctx, mm, Some(filter)).await?;

        Ok(count)
    }
}
