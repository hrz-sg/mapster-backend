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

	// -- Create journey from posts
	let req_create_journey_from_posts = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "create_journey_from_posts",
			"params": {
				"data" : {
					"title": "Journey created from posts",
					"description": "some description",
					"cover_object_key": null,
					"post_ids": vec!["pst_demo1", "pst_demo2", "pst_demo3"], // post ids from seeds
				}
			}
		}),
	);
	let result = req_create_journey_from_posts.await?;
	result.print().await?;
	let created_journey_id: String = result.json_value("/result/id")?;

	// -- Create journey from posts
	let req_create_with_new_post = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 2,
			"method": "create_with_new_post",
			"params": {
				"data" : {
					"existing_post_id": "pst_demo4",
					"title": "Journey created with new post",
					"description": "some description",
					"new_post": {
						"title": "post 1",
						"description": "description 1",
						"status": "Published",
						"cover_media_key": "post/pst_test123/dsd.jpg"
					}
				}
			}
		}),
	);
	let result = req_create_with_new_post.await?;
	result.print().await?;

	// -- Create journey from posts
	let req_detach_post_from_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 3,
			"method": "detach_post_from_journey",
			"params": {
				"data" : {
					"journey_id": created_journey_id,
					"post_id": "pst_demo3",
				}
			}
		}),
	);
	let result = req_detach_post_from_journey.await?;
	result.print().await?;

	// -- Get journey metadata
	let req_get_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 4,
			"method": "get_journey",
			"params": {
				"id": created_journey_id,
			}
		}),
	);
	let result = req_get_journey.await?;
	result.print().await?;

	// -- Get journey with posts
	let req_get_with_posts_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 5,
			"method": "get_journey_with_posts",
			"params": {
				"id": created_journey_id,
			}
		}),
	);
	let result = req_get_with_posts_journey.await?;
	result.print().await?;

	// -- Update journey
	let req_update_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 6,
			"method": "update_journey",
			"params": {
				"id": created_journey_id,
				"data" : {
					"title": "updated title",
					"description": "updated description",
					"status": "Published"
				}
			}
		}),
	);
	let result = req_update_journey.await?;
	result.print().await?;

	// -- Delete journey
	let req_update_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 6,
			"method": "delete_journey",
			"params": {
				"id": created_journey_id
			}
		}),
	);
	let result = req_update_journey.await?;
	result.print().await?;
	
	// -- List journeys by user and published
	let req_list_journey = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 7,
			"method": "list_journey",
			"params": {
				"filters": {
					"owner_id": "usr_demo0" ,
					"status": "Published"
				}
			}
		}),
	);
	let result = req_list_journey.await?;
	result.print().await?;
	let first_journey_id: String = result.json_value("/result/0/id")?;

	// -- Add post to journey end
	let req_add_post_to_journey_end = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 8,
			"method": "add_post_to_journey_end",
			"params": {
				"id": first_journey_id,
				"data" : {
					"post_id": "pst_demo3"
				}
			}
		}),
	);
	let result = req_add_post_to_journey_end.await?;
	result.print().await?;

	// -- Move post position
	let req_move_post_position = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 9,
			"method": "move_post_position",
			"params": {
				"id": first_journey_id,
				"data" : {
					"post_id": "pst_demo3",
					"sort_order": 1,
				}
			}
		}),
	);
	let result = req_move_post_position.await?;
	result.print().await?;

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