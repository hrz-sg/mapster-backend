use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::journey::{
    JourneyPost, JourneyPostBmc, JourneyPostFilter, JourneyPostForCreate, JourneyPostForUpdate,
};
use crate::service::error::{Error, Result};
use modql::filter::ListOptions;
use std::collections::HashMap;
use tracing::{error, info};

pub struct JourneyPostService;

impl JourneyPostService {
    /// --- Add post into journey
    pub async fn add_post_to_journey_end(ctx: &Ctx, mm: &ModelManager, journey_id: &str, post_id: &str) -> Result<()> {
        info!("Starting to add post: {} to journey: {}", post_id, journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if post already in journey
        if JourneyPostBmc::exists(ctx, &mm_txn, journey_id, post_id).await? {
            dbx.rollback_txn().await?;
            info!("Post {} already in journey {}, rolling back", post_id, journey_id);
            return Err(Error::already_exists("Post already in journey"));
        }

        // -- Get posts listed by journey id
        let current_posts = Self::list_by_journey(ctx, &mm_txn, journey_id).await?;
        info!("Journey {} currently has {} posts", journey_id, current_posts.len());

        // -- Compute the next order (in the end)
        let next_order = current_posts.last().map(|post| post.sort_order + 1).unwrap_or(0);
        info!("Next order position: {}", next_order);

        // -- Prepare data to insert post into journey
        let journey_post_c = JourneyPostForCreate {
            journey_id: journey_id.to_string(),
            post_id: post_id.to_string(),
            sort_order: next_order,
        };

        // -- Insert post into the last index in journey
        JourneyPostBmc::create(ctx, &mm_txn, journey_post_c).await?;
        info!("Post: {} added to journey: {}", post_id, journey_id);

        dbx.commit_txn().await?;

        Ok(())
    }

    /// --- Reorder one post's position
    pub async fn move_post_to_position(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        post_id: &str,
        new_position: i32,
    ) -> Result<()> {
        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if post exists and its position
        let current_jp = JourneyPostBmc::get(ctx, &mm_txn, journey_id, post_id).await?;
        info!("Current position of post {}: {}", post_id, current_jp.sort_order);

        // -- If already on the correct position
        if current_jp.sort_order == new_position {
            dbx.rollback_txn().await?;
            info!("Post {} already at position {}", post_id, new_position);
            return Ok(());
        }

        // -- Gell all posts in journey
        let all_posts = Self::list_by_journey(ctx, &mm_txn, journey_id).await?;
        info!("Journey {} has {} total posts", journey_id, all_posts.len());

        // -- Check validation of new_position
        // After delete current post, there are left (all_posts.len() - 1) posts
        // New positions: 0..(all_posts.len() - 1)
        if new_position < 0 || new_position >= all_posts.len() as i32 {
            dbx.rollback_txn().await?;
            error!(
                "Invalid position {} for journey with {} posts",
                new_position,
                all_posts.len()
            );
            return Err(Error::validation_failed(format!(
                "Invalid position: {}. Must be between 0 and {}",
                new_position,
                all_posts.len() - 1
            )));
        }

        // -- Create new order
        let mut new_order: Vec<String> = all_posts.iter().map(|jp| jp.post_id.clone()).collect();

        // -- Delete from current position
        new_order.remove(current_jp.sort_order as usize);

        // -- Insert into new position
        // Now new_order contains (all_posts.len() - 1) elements
        new_order.insert(new_position as usize, post_id.to_string());

        info!("New order for journey {}: {:?}", journey_id, new_order);

        // -- Apply new order
        // TODO: improve
        Self::reorder_posts_in_journey(ctx, &mm_txn, journey_id, new_order).await?;

        info!(
            "Post {} moved from position {} to {} in journey {}",
            post_id, current_jp.sort_order, new_position, journey_id
        );

        // -- Complete transaction
        dbx.commit_txn().await?;

        Ok(())
    }

    /// --- Rearrange posts positions in journey
    /// Note: should accept transaction
    pub async fn reorder_posts_in_journey(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        new_order: Vec<String>,
    ) -> Result<()> {
        // -- Load current posts once
        let current_posts = Self::list_by_journey(ctx, mm, journey_id).await?;

        Self::reorder_posts_internal(ctx, mm, journey_id, &current_posts, &new_order).await
    }

    async fn reorder_posts_internal(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        current_posts: &[JourneyPost],
        new_order: &[String],
    ) -> Result<()> {
        info!("Reordering {} posts in journey {}", new_order.len(), journey_id);

        // -- Validation: unique ids
        let mut seen = HashMap::with_capacity(new_order.len());
        for (idx, post_id) in new_order.iter().enumerate() {
            if seen.insert(post_id.as_str(), idx).is_some() {
                return Err(Error::validation_failed("Duplicate posts in order list"));
            }
        }

        // -- Build map: post_id -> current JourneyPost
        let current_map: HashMap<&str, &JourneyPost> =
            current_posts.iter().map(|jp| (jp.post_id.as_str(), jp)).collect();

        // -- Validate all posts exist
        if current_map.len() != new_order.len() {
            return Err(Error::validation_failed(
                "New order does not match current journey posts",
            ));
        }

        // -- Apply updates
        let mut updated = 0;

        for (new_position, post_id) in new_order.iter().enumerate() {
            let current = current_map.get(post_id.as_str()).ok_or_else(|| {
                Error::validation_failed(format!("Post {} is not part of journey {}", post_id, journey_id))
            })?;

            let new_position = new_position as i32;

            if current.sort_order != new_position {
                JourneyPostBmc::update(
                    ctx,
                    mm,
                    journey_id,
                    post_id,
                    JourneyPostForUpdate {
                        sort_order: new_position,
                    },
                )
                .await?;

                updated += 1;
            }
        }

        info!("Updated {} post positions in journey {}", updated, journey_id);

        Ok(())
    }

    // helper function
    pub async fn list_by_journey(ctx: &Ctx, mm: &ModelManager, journey_id: &str) -> Result<Vec<JourneyPost>> {
        // -- Filter by journey_id
        let filters = Some(vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            post_id: None,
        }]);

        // -- Sort by sort order
        let list_options = Some(ListOptions {
            order_bys: Some("sort_order".into()),
            ..Default::default()
        });

        let posts = JourneyPostBmc::list(ctx, mm, filters, list_options).await?;

        Ok(posts)
    }
}
