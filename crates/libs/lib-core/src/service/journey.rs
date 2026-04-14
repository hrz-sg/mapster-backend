// region: ---- Imports

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::journey::{Journey, JourneyBmc, JourneyFilter, JourneyForCreate, JourneyForUpdate, JourneyStatus};
use crate::model::journey_collection::JourneyCollectionBmc;
use crate::model::journey_collection_item::{JourneyCollectionItemBmc, JourneyCollectionItemFilter};
use crate::model::journey_forward::{JourneyForwardBmc, JourneyForwardFilter, JourneyForwardForCreate};
use crate::model::journey_post::{JourneyPost, JourneyPostBmc, JourneyPostFilter, JourneyPostForCreate};
use crate::model::post::{Post, PostBmc, PostFilter, PostForCreate, PostWithAuthor};
use crate::service::error::{Error, Result};
use crate::service::journey_post::JourneyPostService;
use modql::filter::{ListOptions, OpValString, OpValsString};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{info, warn};

// endregion: ---- Imports

pub struct JourneyService;

impl JourneyService {
    /// --- Create journey. Scenario 1: Create journey from existing posts (main way of journey creation)
    pub async fn create_from_posts(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreateJourneyWithPostsPayload
    ) -> Result<Journey> {

        let CreateJourneyWithPostsPayload {
            title,
            description,
            cover_object_key,
            post_ids,
        } = payload;
        
        info!(
            "Creating journey from {} selected posts for user: {}",
            post_ids.len(),
            ctx.user_id()
        );

        if post_ids.len() < 2 {
            return Err(Error::validation_failed("Journey must have at least 2 posts"));
        }

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // let mut first_post_cover_object_key: Option<String> = None;

        let post_filter = vec![PostFilter {
            id: Some(OpValsString(vec![OpValString::In(post_ids.clone())])),
            ..Default::default()
        }];

        let posts = PostBmc::list(ctx, &mm_txn, Some(post_filter), None).await?;

        let found_ids: std::collections::HashSet<_> =
            posts.iter().map(|p| &p.id).collect();

        let missing_ids: Vec<&str> = post_ids
            .iter()
            .filter(|id| !found_ids.contains(id))
            .map(|s| s.as_str())
            .collect();

        if !missing_ids.is_empty() {
            return Err(Error::entity_not_found(
                "Post",
                missing_ids.join(", ")
            ));
        }

        let journey_post_filter = vec![JourneyPostFilter {
            post_id: Some(OpValsString(vec![OpValString::In(post_ids.clone())])),
            ..Default::default()
        }];

        let existing_journey_posts = JourneyPostBmc::list(ctx, &mm_txn, Some(journey_post_filter), None).await?;

        if !existing_journey_posts.is_empty() {
            return Err(Error::validation_failed(
                "One or more posts already belong to a journey"
            ));
        }

        let mut first_post_cover_object_key: Option<String> = None;

        let posts_map: std::collections::HashMap<_, _> =
            posts.into_iter().map(|p| (p.id.clone(), p)).collect();

        for (index, post_id) in post_ids.iter().enumerate() {
            let post = posts_map.get(post_id).expect("Missing ids already checked");

            if post.owner_id != ctx.user_id() {
                return Err(Error::permission_denied(
                    "Cannot include other user's posts in your journey",
                ));
            }

            if index == 0 && cover_object_key.is_none() && !post.cover_media_key.is_empty() {
                first_post_cover_object_key = Some(post.cover_media_key.clone());
            }
        }

        // Priority:
        // 1. cover from payload
        // 2. cover from first post
        // 3. None
        let final_cover_object_key =
            cover_object_key.or(first_post_cover_object_key);

        // -- Create journey
        let journey_c = JourneyForCreate {
            title,
            description,
            cover_object_key: final_cover_object_key,
            status: JourneyStatus::Published, 
        };

        let journey_id = JourneyBmc::create(ctx, &mm_txn, journey_c).await?;

        // -- Add posts into journey
        for (index, post_id) in post_ids.iter().enumerate() {
            let journey_post_c = JourneyPostForCreate {
                journey_id: journey_id.clone(),
                post_id: post_id.to_string(),
                sort_order: index as i32,
            };
            JourneyPostBmc::create(ctx, &mm_txn, journey_post_c).await?;
        }

        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        dbx.commit_txn().await?;

        info!(
            "Journey created successfully: {}",
            journey_id
        );

        Ok(journey)
    }

    /// --- Create journey.
    /// Scenario 2: "Continue as a journey" from existing post
    /// User creates new post, binds with existing post -> Creates new journey
    ///
    /// Scenario 3: "Bind with existing post" when creating post
    /// Creates journey from current post + new post
    ///
    /// NOTE: both scenarios are used in this method, it depends on frontend
    pub async fn create_with_new_post(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreateJourneyWithNewPostPayload
    ) -> Result<(Journey, Post)> {

        info!(
            "Continuing as journey from post for user: {}",
            ctx.user_id()
        );

        let CreateJourneyWithNewPostPayload {
            existing_post_id,
            title,
            description,
            cover_object_key,
            new_post,
        } = payload;

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let existing_post = PostBmc::get(ctx, &mm_txn, &existing_post_id).await?;

        if existing_post.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("Post doesn't belong to you"));
        }

        // -- Check if post already exists in journey
        if JourneyPostBmc::find_journey_id_by_post(ctx, &mm_txn, &existing_post_id)
            .await?
            .is_some()
        {
            return Err(Error::validation_failed("Post is already in a journey"));
        }

        // -- Create new post
        let new_post_id = PostBmc::create(ctx, &mm_txn, new_post).await?;
        let new_post = PostBmc::get(ctx, &mm_txn, &new_post_id).await?;

        // -- Define cover
        let final_cover_object_key = cover_object_key
        .or_else(|| (!existing_post.cover_media_key.is_empty()).then(|| existing_post.cover_media_key.clone()))
        .or_else(|| (!new_post.cover_media_key.is_empty()).then(|| new_post.cover_media_key.clone()));

        let journey_c = JourneyForCreate {
            title,
            description,
            cover_object_key: final_cover_object_key, 
            status: JourneyStatus::Published, // publish by default
        };

        let journey_id = JourneyBmc::create(ctx, &mm_txn, journey_c).await?;

        // -- Add two posts into journey
        let journey_post1 = JourneyPostForCreate {
            journey_id: journey_id.clone(),
            post_id: existing_post_id.to_string(),
            sort_order: 0,
        };
        JourneyPostBmc::create(ctx, &mm_txn, journey_post1).await?;

        let journey_post2 = JourneyPostForCreate {
            journey_id: journey_id.clone(),
            post_id: new_post_id.clone(),
            sort_order: 1,
        };
        JourneyPostBmc::create(ctx, &mm_txn, journey_post2).await?;

        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        dbx.commit_txn().await?;

        info!(
            "Journey {} created continuing from post {} with new post {}",
            journey_id, existing_post_id, new_post_id
        );

        Ok((journey, new_post))
    }

    /// --- Detatch post from journey (post remains but is removed from journey)
    pub async fn detach_post_from_journey(
        ctx: &Ctx, 
        mm: &ModelManager, 
        payload: DetachPostPayload,
    ) -> Result<()> {

        let DetachPostPayload {
            journey_id,
            post_id,
        } = payload;
        
        info!("Detaching post {} from journey {}", post_id, journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if user is the journey owner
        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        if journey.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("You are not the owner of this journey"));
        }

        let current_posts = JourneyPostService::list_by_journey(ctx, &mm_txn, &journey_id).await?;

        // Check minimum posts length (2)
        if current_posts.len() - 1 < 2 {
            return Err(Error::validation_failed("Journey must contain at least 2 posts"));
        }

        // -- Remove post relation to journey
        JourneyPostBmc::delete(ctx, &mm_txn, &journey_id, &post_id).await?;

        // -- Reorder left posts
        let remaining_posts = JourneyPostService::list_by_journey(ctx, &mm_txn, &journey_id).await?;
        let new_order: Vec<String> = remaining_posts.into_iter().map(|jp| jp.post_id).collect();

        JourneyPostService::reorder_posts_in_journey(ctx, &mm_txn, &journey_id, new_order).await?;

        dbx.commit_txn().await?;

        info!("Post {} detached from journey {}", post_id, journey_id);

        Ok(())
    }

    /// --- Get signle journey metadata (without posts)
    pub async fn get(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<Journey> {
        info!("Getting journey: {}", journey_id);

        let journey = JourneyBmc::get(ctx, mm, &journey_id).await?;

        let is_owner = journey.owner_id == ctx.user_id();

        match (is_owner, journey.status.clone()) {
            // Owner can see everything
            (true, _) => Ok(journey),

            // Others can see only published
            (false, JourneyStatus::Published) => Ok(journey),

            // Everything else forbidden
            _ => Err(Error::permission_denied(
                "You don't have permission to view this journey",
            )),
        }
    }

    pub async fn get_with_posts(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: String,
    ) -> Result<(Journey, Vec<PostWithAuthor>)> {

        info!("Getting journey with posts: {}", journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Get journey
        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        // -- Access check
        let is_owner = journey.owner_id == ctx.user_id();

        if !is_owner && journey.status != JourneyStatus::Published {
            return Err(Error::permission_denied(
                "You don't have permission to view this journey",
            ));
        }

        // -- Get journey posts
        let post_filters = Some(vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }]);

        let list_options = Some(ListOptions {
            order_bys: Some("sort_order".to_string().into()),
            ..Default::default()
        });

        let journey_posts =
            JourneyPostBmc::list(ctx, &mm_txn, post_filters, list_options).await?;

        let posts_with_user_preview =
            Self::get_posts_details(ctx, &mm_txn, &journey_posts).await?;

        dbx.commit_txn().await?;

        Ok((journey, posts_with_user_preview))
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: String,
        journey_u: JourneyForUpdate,
    ) -> Result<Journey> {

        info!("Updating journey: {}", journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if journey exists and user has permission
        let existing_journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        if existing_journey.owner_id != ctx.user_id() {
            return Err(Error::permission_denied(
                "You don't have permission to edit this journey",
            ));
        }

        // -- Update journey
        JourneyBmc::update(ctx, &mm_txn, &journey_id, journey_u).await?;

        // -- Get updated journey
        let updated_journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        // -- Commit transaction
        dbx.commit_txn().await?;

        info!("Journey updated successfully: {}", journey_id);
        Ok(updated_journey)
    }

    /// --- Delete journey (cascade deletes journey_posts)
    pub async fn delete(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<()> {
        info!("Deleting journey: {}", journey_id);

        // Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // Check if journey exists and user has permission
        let existing_journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        if existing_journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied(
                "You don't have permission to delete this journey",
            ));
        }

        // Delete journey (cascade will delete all related journey_posts)
        JourneyBmc::delete(ctx, &mm_txn, &journey_id).await?;

        // Commit transaction
        dbx.commit_txn().await?;

        info!("Journey deleted successfully: {}", journey_id);
        Ok(())
    }

    /// --- Check if current user saved the journey
    pub async fn has_current_user_saved(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<bool> {
        // -- Find user collection
        let collection = match JourneyCollectionBmc::find_default(ctx, mm, ctx.user_id()).await? {
            Some(col) => col,
            None => return Ok(false),
        };

        // -- Check if journey exists in collection
        let exists = JourneyCollectionItemBmc::exists_in_collection(ctx, mm, &collection.id, &journey_id).await?;

        Ok(exists)
    }

    /// --- Get journeys saved by current user
    pub async fn list_saved_journeys(
        ctx: &Ctx,
        mm: &ModelManager,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Journey>> {
        info!("Listing saved journeys for user: {}", ctx.user_id());

        // -- Find user collection
        let collection = match JourneyCollectionBmc::find_default(ctx, mm, ctx.user_id()).await? {
            Some(col) => col,
            None => return Ok(vec![]), // no collection - no data
        };

        // Get journey ids from collection
        let items = JourneyCollectionItemBmc::list_in_collection(ctx, mm, &collection.id).await?;

        let journey_ids: Vec<&str> = items.iter().map(|item| item.journey_id.as_str()).collect();

        if journey_ids.is_empty() {
            return Ok(vec![]);
        }

        let journey_ids_opvals: Vec<OpValString> = journey_ids.iter().map(|id| OpValString::from(*id)).collect();

        // Get journeys details
        let filters = vec![JourneyFilter {
            id: Some(journey_ids_opvals.into()), // IN filter
            status: Some(JourneyStatus::Published),     // only published
            ..Default::default()
        }];

        let list_options = ListOptions {
            limit,
            offset,
            order_bys: Some("-ctime".to_string().into()), // new first
        };

        let journeys = JourneyBmc::list(ctx, mm, Some(filters), Some(list_options)).await?;

        Ok(journeys)
    }

    /// --- Check if current user forwarded the journey
    pub async fn has_current_user_forwarded(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<bool> {
        let is_forwarded = JourneyForwardBmc::exists(ctx, mm, &journey_id, ctx.user_id()).await?;

        Ok(is_forwarded)
    }

    /// --- Save journey to user's default collection
    pub async fn save_journey(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<()> {
        info!("User {} saving journey: {}", ctx.user_id(), journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check journey exists and accessible
        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        // -- Save only published journeys 
        let is_owner = journey.owner_id == ctx.user_id();

        if !is_owner && journey.status != JourneyStatus::Published {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot save unpublished journey"));
        }

        // -- Get or create default collection for user
        let collection = JourneyCollectionBmc::get_or_create_default(ctx, &mm_txn, ctx.user_id()).await?;

        // -- Add journey to collection (ON CONFLICT DO NOTHING)
        let was_added = JourneyCollectionItemBmc::add_to_collection(ctx, &mm_txn, &collection.id, &journey_id).await?;

        // -- Update counter only if actually added (not a duplicate)
        if was_added {
            JourneyBmc::increment_save_count(ctx, &mm_txn, &journey_id).await?;
        }

        // -- Commit transaction
        dbx.commit_txn().await?;

        info!("Journey {} saved successfully", journey_id);
        Ok(())
    }

    /// --- Remove journey from user's default collection  
    pub async fn unsave_journey(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<()> {
        info!("User {} unsaving journey: {}", ctx.user_id(), journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check journey exists
        JourneyBmc::exists(ctx, &mm_txn, &journey_id).await?;

        // -- Find user's default collection
        let collection = match JourneyCollectionBmc::find_default(ctx, &mm_txn, ctx.user_id()).await? {
            Some(col) => col,
            None => {
                dbx.rollback_txn().await?;
                return Err(Error::entity_not_found("Journey Collection", ctx.user_id().to_string()));
            }
        };

        // -- Remove journey from collection
        JourneyCollectionItemBmc::remove_from_collection(ctx, &mm_txn, &collection.id, &journey_id).await?;

        // -- Decrement save count
        JourneyBmc::decrement_save_count(ctx, &mm_txn, &journey_id).await?;

        dbx.commit_txn().await?;

        info!("Journey {} unsaved successfully", journey_id);
        Ok(())
    }

    /// --- List journeys
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<JourneyFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Journey>> {
        info!("Listing journeys with filters");

        let journeys = JourneyBmc::list(ctx, mm, filters, list_options).await?;

        Ok(journeys)
    }

    /// --- Forward journey to chat
    pub async fn forward(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String, 
        chat_id: String
    ) -> Result<()> {
        info!(
            "User {} forwarding journey {} to chat {}",
            ctx.user_id(),
            journey_id,
            chat_id
        );

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;

        let is_owner = journey.owner_id == ctx.user_id();
        if !is_owner && journey.status != JourneyStatus::Published {
            return Err(Error::permission_denied("Cannot forward unpublished journey"));
        }

        let journey_forward_c = JourneyForwardForCreate {
            journey_id: journey_id.to_string(),
            chat_id: chat_id.to_string(),
        };

        // -- Creates relation in table between forwarded_journey and user_id
        // DOES NOTHING ON CONFLICT
        JourneyForwardBmc::create_on_conflict(ctx, &mm_txn, journey_forward_c).await?;

        // -- Increment forward count
        JourneyBmc::increment_forward_count(ctx, &mm_txn, &journey_id).await?;

        dbx.commit_txn().await?;

        info!("Journey {} forwarded successfully", journey_id);

        Ok(())
    }

    /// --- Unforward journey
    pub async fn unforward(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<()> {
        info!("User {} unforwarding journey: {}", ctx.user_id(), journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        JourneyBmc::exists(ctx, &mm_txn, &journey_id).await?;

        JourneyForwardBmc::delete(ctx, &mm_txn, &journey_id).await?;

        // -- Decrement forward count
        JourneyBmc::decrement_forward_count(ctx, &mm_txn, &journey_id).await?;

        dbx.commit_txn().await?;

        info!("Journey {} unforwarded successfully", journey_id);
        Ok(())
    }

    // region: --- Helpers

    /// --- Count forwards for the journey
    pub async fn count_forwards(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<i64> {
        let filter = vec![JourneyForwardFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }];
        let count = JourneyForwardBmc::count(ctx, mm, Some(filter)).await?;

        Ok(count)
    }

    /// --- Count saves for the journey
    pub async fn count_saves(
        ctx: &Ctx, 
        mm: &ModelManager, 
        journey_id: String
    ) -> Result<i64> {
        // We count ALL collections that contain this trip
        // (not just the default ones)

        let filter = vec![JourneyCollectionItemFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }];

        let count = JourneyCollectionItemBmc::count(ctx, mm, Some(filter)).await?;

        Ok(count)
    }

    /// --- Helper: Get posts details with users
    async fn get_posts_details(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_posts: &[JourneyPost],
    ) -> Result<Vec<PostWithAuthor>> {
        if journey_posts.is_empty() {
            return Ok(vec![]);
        }

        // -- Get post IDs
        let post_ids: Vec<&str> = journey_posts.iter().map(|jp| jp.post_id.as_str()).collect();

        // -- Get posts with user info
        let posts = PostBmc::get_many_with_authors(ctx, mm, post_ids).await?;

        // -- Create HashMap for quick lookup
        let posts_map: HashMap<String, PostWithAuthor> = posts.into_iter().map(|post| (post.id.clone(), post)).collect();

        // Use Vec::with_capacity() for better performance
        let mut ordered_posts = Vec::with_capacity(journey_posts.len());

        for journey_post in journey_posts {
            match posts_map.get(&journey_post.post_id) {
                Some(post) => ordered_posts.push(post.clone()),
                None => {
                    // Post might have been deleted but relationship remains
                    warn!(
                        "Post {} not found but referenced in journey {}. Skipping.",
                        journey_post.post_id, journey_post.journey_id
                    );
                }
            }
        }

        Ok(ordered_posts)
    }

    // endregion: --- Helpers

}

// region: --- Payloads
#[derive(Debug, Deserialize)]
pub struct CreateJourneyWithPostsPayload {
    pub title: String,
    pub description: String,
    pub cover_object_key: Option<String>,
    pub post_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJourneyWithNewPostPayload {
    pub existing_post_id: String,
    pub title: String,
    pub description: String,
    pub cover_object_key: Option<String>,
    pub new_post: PostForCreate,
}

#[derive(Debug, Deserialize)]
pub struct DetachPostPayload {
    pub journey_id: String,
    pub post_id: String,
}

// endregion: --- Payloads
