#![allow(unused)]

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// -- Login
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo0",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	// -- Create post with post media metadata
	let req_create_post_with_meta = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "create_post_with_media_metadata",
			"params": {
				"data": {
					"title": "post 1",
					"description": "description 1",
					"medias": [
                        {
                            "object_key": "post/pst_test123/dsd.jpg",
                            "file_name": "dsd.jpg",
                            "media_type": "Image",
                            "mime_type": "image/jpg",
                            "width": 300,
                            "height": 300,
                            "duration": null
                        },
                    ]
				}
			}
		}),
	);
	let result = req_create_post_with_meta.await?;
	result.print().await?;
	let created_post_id: String = result.json_value("/result")?;

	// -- Create post with post media metadata
	let req_create_post_with_meta = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "create_post_with_media_metadata",
			"params": {
				"data": {
					"title": "post 1",
					"description": "description 1",
					"medias": [
                        {
                            "object_key": "post/pst_test123/dsd.jpg",
                            "file_name": "dsd.jpg",
                            "media_type": "Image",
                            "mime_type": "image/jpg",
                            "width": 300,
                            "height": 300,
                            "duration": null
                        },
                    ]
				}
			}
		}),
	);
	let result = req_create_post_with_meta.await?;
	result.print().await?;
	let created_post_id: String = result.json_value("/result")?;

	// // -- Get feed posts
	// let req_list_feed = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 3,
	// 		"method": "list_feed_posts",
	// 		"params": {
	// 			"filters": {},
	// 			"list_options": { "limit": 10, "offset": 0 } 
	// 		}
	// 	}),
	// );
	// let result = req_list_feed.await?;
	// result.print().await?;
	// let post_detail_id: String = result.json_value("/result/0/id")?;

	// // -- Get post detail
	// let req_get_post = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 2,
	// 		"method": "get_post_detail",
	// 		"params": { "id": post_detail_id }
	// 	}),
	// );
	// let result = req_get_post.await?;
	// result.print().await?;

	// -- Get user posts (for profile)
	let req_list_user = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 4,
			"method": "list_user_posts",
			"params": {
				"id": "usr_demo0"
			}
		}),
	);

	let res_list_user = req_list_user.await?;
	res_list_user.print().await?;

	// // -- Create comment
	// let req_create_comment = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "create_comment",
	// 		"params": {
	// 			"data": {
	// 				"post_id": post_detail_id, 
	// 				"text": "created comment 1",
	// 			}
	// 		}
	// 	}),
	// );
	// let result = req_create_comment.await?;
	// result.print().await?;
	// let comment_id: String = result.json_value("/result/id")?;

	// // -- Update comment
	// let req_update_comment = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "update_comment",
	// 		"params": {
	// 			"id": comment_id,
	// 			"data": {
	// 				"text": "updated comment 1.1",
	// 			}
	// 		}
	// 	}),
	// );
	// let result = req_update_comment.await?;
	// result.print().await?;

	// // // -- Delete comment
	// let req_update_comment = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "delete_comment",
	// 		"params": {
	// 			"id": comment_id,
	// 		}
	// 	}),
	// );
	// let result = req_update_comment.await?;
	// result.print().await?;

	// // -- List comments
	// let req_update_comment = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "list_comments",
	// 		"params": {
	// 			"id": post_detail_id,
	// 		}
	// 	}),
	// );
	// let result = req_update_comment.await?;
	// result.print().await?;

	// // -- List comment replies
	// let req_update_comment = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "list_comment_replies",
	// 		"params": {
	// 			"id": comment_id,
	// 		}
	// 	}),
	// );
	// let result = req_update_comment.await?;
	// result.print().await?;

	// // -- Toggle like 
	// let req_toggle_like = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "toggle_like",
	// 		"params": {
	// 			"id": post_detail_id,
	// 		}
	// 	}),
	// );
	// let result = req_toggle_like.await?;
	// result.print().await?;

	// // -- Get likers
	// let req_get_likers = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "get_likers",
	// 		"params": {
	// 			"id": post_detail_id,
	// 		}
	// 	}),
	// );
	// let result = req_get_likers.await?;
	// result.print().await?;

	// // -- Get likes
	// let req_get_likes = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "get_like_count",
	// 		"params": {
	// 			"id": post_detail_id,
	// 		}
	// 	}),
	// );
	// let result = req_get_likes.await?;
	// result.print().await?;

	// // -- Save post to collection
	// let req_save_to_collection = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "save_to_collection",
	// 		"params": {
	// 			"data": {
	// 				"post_id": post_detail_id,
	// 				"option": "Default"
	// 			}
	// 		}
	// 	}),
	// );
	// let result = req_save_to_collection.await?;
	// result.print().await?;
	// let collection_id: String = result.json_value("/result")?;

	// // -- Unsave post from collection
	// let req_save_to_collection = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "unsave_from_collection",
	// 		"params": {
	// 			"data": {
	// 				"post_id": post_detail_id,
	// 				"collection_id": collection_id,
	// 			}
	// 		}
	// 	}),
	// );
	// let result = req_save_to_collection.await?;
	// result.print().await?;

	// // -- Update post
	// let req_update_post = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "update_post_with_media_meta",
	// 		"params": {
	// 			"id": post_detail_id,
	// 			"data": {
	// 				"title": "Updated title",
	// 				"status": "Published"
	// 			}
	// 		}
	// 	}),
	// );
	// let result = req_update_post.await?;
	// result.print().await?;
	
	// // -- Delete post
	// let req_delete_post = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"jsonrpc": "2.0",
	// 		"id": 1,
	// 		"method": "delete_post",
	// 		"params": {
	// 			"id": post_detail_id,
	// 		}
	// 	}),
	// );
	// let result = req_delete_post.await?;
	// result.print().await?;

	// -- Logout
	let req_logout = hc.do_post(
		"/api/logout",
		json!({
			"logout": true
		}),
	);
	req_logout.await?.print().await?;

	Ok(())
}