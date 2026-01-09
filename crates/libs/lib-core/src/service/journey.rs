use std::collections::HashMap;
use modql::filter::ListOptions;
use tracing::{info, warn};
use crate::ctx::Ctx;
use crate::model::journey::{Journey, JourneyBmc, JourneyFilter, JourneyForCreate, JourneyForUpdate};
use crate::model::ModelManager;
use crate::model::journey_forward::{JourneyForwardBmc, JourneyForwardForCreate};
use crate::model::journey_post::{JourneyPost, JourneyPostBmc, JourneyPostFilter, JourneyPostForCreate};
use crate::model::journey_save::{JourneySaveBmc, JourneySaveForCreate};
use crate::model::post::{Post, PostBmc, PostForCreate, PostWithUser};
use crate::service::error::{Error, Result};
use crate::service::journey_post::{list_by_journey, list_by_post, JourneyPostService};

pub struct JourneyService;

impl JourneyService {
    /// --- Create journey. Scenario 1:Create journey from existing posts (main way of journey creation)
    pub async fn create_from_posts(
        ctx: &Ctx,
        mm: &ModelManager,
        title: &str,
        description: &str,
        cover_media_url: Option<&str>,
        post_ids: Vec<&str>
    ) -> Result<Journey> {
        info!("Creating journey from {} selected posts for user: {}", 
            post_ids.len(), ctx.user_id());

        if post_ids.len() < 2 {
            return Err(Error::validation("Journey must have at least 2 posts"));
        }

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let mut first_post_cover_url = None;
        for (index, post_id) in post_ids.iter().enumerate() {
            let post = PostBmc::get(ctx, &mm_txn, post_id).await?;
            
            // -- Check owner
            if post.owner_id != ctx.user_id() {
                dbx.rollback_txn().await?;
                return Err(Error::permission_denied("Cannot include other user's posts in your journey"));
            }

            // -- Check that post is not in other journey
            let journey_posts = list_by_post(ctx, &mm_txn, post_id).await?;
            if !journey_posts.is_empty() {
                dbx.rollback_txn().await?;
                return Err(Error::validation(format!("Post {} is already in another journey", post_id)));
            }

            // -- Save cover_media_url from first post
            if index == 0 && cover_media_url.is_none() && post.cover_media_url.is_some() {
                first_post_cover_url = post.cover_media_url.clone();
            }
        }

        // -- Define cover: priorities:
        //    - given cover_media_url
        //    - cover from first post
        //    - None 
        let final_cover_url = cover_media_url
            .map(|s| s.to_string())
            .or(first_post_cover_url);

        // -- Create journey
        let journey_c = JourneyForCreate {
            title: title.to_string(),
            description: description.to_string(),
            cover_media_url: final_cover_url,
            is_published: Some(true), // publish by default
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

        info!("Journey created from {} posts successfully: {}", post_ids.len(), journey_id);
        Ok(journey)
    }
    
    /// --- Create journey. Scenario 2: "Continue as a journey" from existing post
    /// User creates new post, binds with existing post -> Creates new journey
    pub async fn continue_as_journey_from_post(
        ctx: &Ctx,
        mm: &ModelManager,
        existing_post_id: &str,
        journey_title: &str,
        journey_description: &str,
        journey_cover_url: Option<&str>,
        new_post_data: PostForCreate, // data for new post
    ) -> Result<(Journey, Post)> {
        info!("Continuing as journey from post {} for user: {}", existing_post_id, ctx.user_id());

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let existing_post = PostBmc::get(ctx, &mm_txn, existing_post_id).await?;

        if existing_post.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Post doesn't belong to you"));
        }

        // -- Check if post already exists in journey
        let existing_journey_posts = list_by_post(ctx, &mm_txn, existing_post_id).await?;
        if !existing_journey_posts.is_empty() {
            dbx.rollback_txn().await?;
            return Err(Error::validation("Post is already in a journey"));
        }

        // -- Create new post
        let new_post_id = PostBmc::create(ctx, &mm_txn, new_post_data).await?;
        let new_post = PostBmc::get(ctx, &mm_txn, &new_post_id).await?;

        // -- Define cover
        let final_cover_url = journey_cover_url
            .map(|s|s.to_string())
            .or_else(|| existing_post.cover_media_url.clone())
            .or_else(|| new_post.cover_media_url.clone());

        let journey_c = JourneyForCreate {
            title: journey_title.to_string(),
            description: journey_description.to_string(),
            cover_media_url: final_cover_url,
            is_published: Some(true), // publish by default
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
        
        info!("Journey {} created continuing from post {} with new post {}", 
            journey_id, existing_post_id, new_post_id);
        Ok((journey, new_post))
    }
    
    /// --- Create journey. Scenario 3: "Bind with existing post" when creating post
    /// Creates journey from current post + new post
    pub async fn bind_new_post_to_existing_post(
        ctx: &Ctx,
        mm: &ModelManager,
        existing_post_id: &str,
        journey_title: &str,
        journey_description: &str,
        journey_cover_url: Option<&str>,
        new_post_data: PostForCreate,
    ) -> Result<(Journey, Post)> {
        info!("Binding new post to existing post {} for user: {}", 
            existing_post_id, ctx.user_id());

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let existing_post = PostBmc::get(ctx, &mm_txn, existing_post_id).await?;

        if existing_post.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Post doesn't belong to you"));
        }

        // -- Check if existing post already in journey
        let existing_journey_posts = list_by_post(ctx, &mm_txn, existing_post_id).await?;
        if !existing_journey_posts.is_empty() {
            dbx.rollback_txn().await?;
            return Err(Error::validation("Post is already in a journey"));
        }

        // -- Create new post
        let new_post_id = PostBmc::create(ctx, &mm_txn, new_post_data).await?;
        let new_post = PostBmc::get(ctx, &mm_txn, &new_post_id).await?;

        // -- Define cover
        let final_cover_url = journey_cover_url
            .map(|s| s.to_string())
            .or_else(|| existing_post.cover_media_url.clone())
            .or_else(|| new_post.cover_media_url.clone());

        // -- Create new journey
        let journey_c = JourneyForCreate {
            title: journey_title.to_string(),
            description: journey_description.to_string(),
            cover_media_url: final_cover_url,
            is_published: Some(true), // publish by default
        };
        
        let journey_id = JourneyBmc::create(ctx, &mm_txn, journey_c).await?;

        // -- Add both posts into new journey
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

        // -- Get the final journey
         let journey = JourneyBmc::get(ctx, &mm_txn, &journey_id).await?;
        
        dbx.commit_txn().await?;
        
        info!("Journey {} created binding new post to existing post {}", journey_id, existing_post_id);
        Ok((journey, new_post))
    }
    
    /// --- Detatch post from journey (post remains but is removed from journey)
    pub async fn detach_post_from_journey(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        post_id: &str,
    ) -> Result<()> {
        info!("Detaching post {} from journey {}", post_id, journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if user is the journey owner
        let journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        if journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("You are not the owner of this journey"));
        }

        let current_posts = list_by_journey(ctx, &mm_txn, journey_id).await?;

        // -- Check if post in journey
        let post_in_journey = current_posts.iter().any(|jp| jp.post_id == post_id);
        if !post_in_journey {
            dbx.rollback_txn().await?;
            return Err(Error::not_found("Post not found in journey"));
        }

        // Check minimum posts length (2)
        if current_posts.len() <= 2 {
            return Err(Error::validation(
                "Cannot detach post: journey must have at least 2 posts"
            ));
        }

        // -- Check if post exists in journey
        if !JourneyPostBmc::exists(ctx, &mm_txn, journey_id, post_id).await? {
            return Err(Error::not_found("Post not found in this journey"));
        }

        // -- Remove post relation to journey
        JourneyPostBmc::delete(ctx, &mm_txn, journey_id, post_id).await?;

        // -- Reorder left posts
        let remaining_posts = list_by_journey(ctx, &mm_txn, journey_id).await?;
        let new_order: Vec<String> = remaining_posts
            .into_iter()
            .map(|jp|jp.post_id)
            .collect();

        JourneyPostService::reorder_posts_in_journey(ctx, &mm_txn, journey_id, new_order).await?;

        dbx.commit_txn().await?;

        info!("Post {} detached from journey {}", post_id, journey_id);
        Ok(())
    }
    
    /// --- Get signle journey metadata (without posts)
    pub async fn get(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<Journey> {
        info!("Getting journey: {}", journey_id);
        
        let journey = JourneyBmc::get(ctx, mm, journey_id).await?;
        
        // Check access: if journey is not published, only owner can view it
        if !journey.is_published && journey.owner_id != ctx.user_id() {
            return Err(Error::permission_denied("You don't have permission to view this journey"));
        }
        
        Ok(journey)
    }

    /// --- Get journey with posts
    pub async fn get_with_posts(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<(Journey, Vec<PostWithUser>)> {
        info!("Getting journey with posts: {}", journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Get journey
        let journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;

        // -- Check access
        if !journey.is_published && journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::PermissionDenied("You don't have permission to view this journey".to_owned()));
        }

        // -- Get journey posts
        let post_filters = Some(vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }]);
        
        // -- Get journey posts (already sorted by sort_order)
        let list_options = Some(ListOptions {
            order_bys: Some("sort_order".to_string().into()),
            ..Default::default()
        });

        // -- Get journey posts
        let journey_posts = JourneyPostBmc::list(ctx, &mm_txn, post_filters, list_options).await?;

        // -- Get post details
        let posts_with_user_preview = Self::get_posts_details(ctx, &mm_txn, &journey_posts).await?;

        dbx.commit_txn().await?;

        Ok((journey, posts_with_user_preview))
    }

    /// --- Update journey
    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        journey_u: JourneyForUpdate,
    ) -> Result<Journey> {
        info!("Updating journey: {}", journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check if journey exists and user has permission
        let existing_journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        
        if existing_journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("You don't have permission to edit this journey"));
        }

        // -- Update journey
        JourneyBmc::update(ctx, &mm_txn, journey_id, journey_u).await?;
        
        // -- Get updated journey
        let updated_journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        
        // -- Commit transaction
        dbx.commit_txn().await?;
        
        info!("Journey updated successfully: {}", journey_id);
        Ok(updated_journey)
    }

    /// --- Delete journey (cascade deletes journey_posts)
    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        info!("Deleting journey: {}", journey_id);

        // Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // Check if journey exists and user has permission
        let existing_journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        
        if existing_journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("You don't have permission to delete this journey"));
        }

        // Delete journey (cascade will delete all related journey_posts)
        JourneyBmc::delete(ctx, &mm_txn, journey_id).await?;
        
        // Commit transaction
        dbx.commit_txn().await?;
        
        info!("Journey deleted successfully: {}", journey_id);
        Ok(())
    }

    pub async fn list_by_user(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
        include_unpublished: bool,
    ) -> Result<Vec<Journey>> {
        info!("Listing journeys for user: {}", user_id);

        let mut filters = vec![JourneyFilter {
            owner_id: Some(user_id.into()),
            ..Default::default()
        }];
        
        // If not requesting unpublished journeys and not viewing own journeys
        let is_viewing_own_journeys = user_id == ctx.user_id();
        
        if !include_unpublished && !is_viewing_own_journeys {
            filters.push(JourneyFilter {
                is_published: Some(true.into()),
                ..Default::default()
            });
        }
        
        let list_options = Some(ListOptions {
            order_bys: Some(vec!["-mtime".to_string()].into()), // Sort by modification time
            ..Default::default()
        });
        
        Self::list(ctx, mm, Some(filters), list_options).await
    }

    /// --- Save journey (and increment saved count)
    pub async fn save(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        info!("User {} saving journey: {}", ctx.user_id(), journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check journey exists and accessible
        let journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        
        // -- Save only published journeys
        if !journey.is_published && journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot save unpublished journey"));
        }
        
        // -- Save journey
        if JourneySaveBmc::exists(ctx, &mm_txn, journey_id, ctx.user_id()).await? {
            info!("Journey {} already saved by user {}", journey_id, ctx.user_id());
            dbx.rollback_txn().await?;
            return Ok(());
        };

        // -- Prepare struct to save relations between journey_id & ctx.user_id()
        let journey_save_c = JourneySaveForCreate {
            journey_id: journey_id.to_string(),
            user_id: ctx.user_id().to_string(),
        };

        // -- Create realtions between 
        JourneySaveBmc::create(ctx, &mm_txn, journey_save_c).await?;

        // -- Update counter in journey by +1
        JourneyBmc::increment_saved_count(ctx, &mm_txn, journey_id).await?;
        
        // -- Commit transaction
        dbx.commit_txn().await?;
        
        info!("Journey {} saved successfully", journey_id);
        Ok(())
    }

    /// --- Unsave journey
    pub async fn unsave(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        info!("User {} unsaving journey: {}", ctx.user_id(), journey_id);

        // -- Start transaction
        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        // -- Check journey exists
        JourneyBmc::exists(ctx, &mm_txn, journey_id).await?;
        
        // -- Unsave journey
        JourneySaveBmc::delete(ctx, &mm_txn, journey_id).await?;

        // -- Decrement save count by -1
        JourneyBmc::decrement_saved_count(ctx, &mm_txn, journey_id).await?;
        
        dbx.commit_txn().await?;
        
        info!("Journey {} unsaved successfully", journey_id);
        Ok(())
    }

    /// --- List journeys with filtering and pagination (only published)
    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<JourneyFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Journey>> {
        info!("Listing journeys with filters");
        
        let mut final_filters = filters.unwrap_or_default();

        if true {  // always show published only
            final_filters.push(JourneyFilter {
                is_published: Some(true.into()),
                ..Default::default()
            });
        }
        
        let journeys = JourneyBmc::list(ctx, mm, Some(final_filters), list_options).await?;
        
        Ok(journeys)
    }
 
    /// --- Forward journey
    pub async fn forward(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
        forward_to_user_id: &str,
    ) -> Result<()> {
        info!("User {} forwarding journey {} to {}", 
            ctx.user_id(), journey_id, forward_to_user_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let journey = JourneyBmc::get(ctx, &mm_txn, journey_id).await?;
        
        if !journey.is_published && journey.owner_id != ctx.user_id() {
            dbx.rollback_txn().await?;
            return Err(Error::permission_denied("Cannot forward unpublished journey"));
        }
        
        if JourneyForwardBmc::exists(ctx, &mm_txn, journey_id, ctx.user_id()).await? {
            info!("Journey {} already forwarded by user {}", journey_id, ctx.user_id());
            dbx.rollback_txn().await?;
            return Ok(());
        }
        
        let journey_forward_c = JourneyForwardForCreate {
            journey_id: journey_id.to_string(),
            user_id: ctx.user_id().to_string(),
            forward_to_user_id: forward_to_user_id.to_string(),
        };

        JourneyForwardBmc::create(ctx, &mm_txn, journey_forward_c).await?;
        
        // -- Increment forward count
        JourneyBmc::increment_forward_count(ctx, &mm_txn, journey_id).await?;
        
        dbx.commit_txn().await?;
        
        info!("Journey {} forwarded successfully", journey_id);
        Ok(())
    }

    /// --- Unforward journey
    pub async fn unforward(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        info!("User {} unforwarding journey: {}", ctx.user_id(), journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        JourneyBmc::exists(ctx, &mm_txn, journey_id).await?;
        
        JourneyForwardBmc::delete(ctx, &mm_txn, journey_id).await?;
        
        // -- Decrement forward count
        JourneyBmc::decrement_forward_count(ctx, &mm_txn, journey_id).await?;
        
        dbx.commit_txn().await?;
        
        info!("Journey {} unforwarded successfully", journey_id);
        Ok(())
    }

    /// --- Update total_likes when likes changed in post
    pub async fn update_total_likes_from_posts(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_id: &str,
    ) -> Result<()> {
        info!("Updating total likes for journey: {}", journey_id);

        let mm_txn = mm.new_with_txn()?;
        let dbx = mm_txn.dbx();
        dbx.begin_txn().await?;

        let post_filters = Some(vec![JourneyPostFilter {
            journey_id: Some(journey_id.into()),
            ..Default::default()
        }]);
        
        let journey_posts = JourneyPostBmc::list(ctx, &mm_txn, post_filters, None).await?;
        
        let mut total_likes: i64 = 0;
        for journey_post in &journey_posts {
            if let Ok(post) = PostBmc::get(ctx, &mm_txn, &journey_post.post_id).await {
                total_likes += post.like_count;
            }
        }
        
        // -- Set absoulte value
        JourneyBmc::set_total_likes(ctx, &mm_txn, journey_id, total_likes).await?;
        
        dbx.commit_txn().await?;
        
        info!("Total likes for journey {} updated to {}", journey_id, total_likes);
        Ok(())
    }

    /// --- Helper: Get posts details with users
    async fn get_posts_details(
        ctx: &Ctx,
        mm: &ModelManager,
        journey_posts: &[JourneyPost],
    ) -> Result<Vec<PostWithUser>> {
        if journey_posts.is_empty() {
            return Ok(vec![]);
        }

        // -- Get post IDs
        let post_ids: Vec<&str> = journey_posts.iter()
            .map(|jp| jp.post_id.as_str())
            .collect();

        // -- Get posts with user info
        let posts = PostBmc::get_many_with_users(ctx, mm, post_ids).await?;

        // -- Create HashMap for quick lookup
        let posts_map: HashMap<String, PostWithUser> = posts
            .into_iter()
            .map(|post| (post.id.clone(), post))
            .collect();

        // Use Vec::with_capacity() for better performance
        let mut ordered_posts = Vec::with_capacity(journey_posts.len());

        for journey_post in journey_posts {
            match posts_map.get(&journey_post.post_id) {
                Some(post) => ordered_posts.push(post.clone()),
                None => {
                    // Post might have been deleted but relationship remains
                     warn!(
                        "Post {} not found but referenced in journey {}. Skipping.",
                        journey_post.post_id,
                        journey_post.journey_id
                    );
                }
            }
        }

        Ok(ordered_posts)
    }
    
    /// ---
    pub async fn get_journey_for_post(
        ctx: &Ctx,
        mm: &ModelManager,
        post_id: &str,
    ) -> Result<Option<Journey>> {
        let journey_posts = list_by_post(ctx, mm, post_id).await?;
        
        match journey_posts.first() {
            Some(journey_post) => {
                let journey = JourneyBmc::get(ctx, mm, &journey_post.journey_id).await?;
                Ok(Some(journey))
            }
            None => Ok(None),
        }
    }
}
