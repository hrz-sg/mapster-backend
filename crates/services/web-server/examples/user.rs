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

	// -- Get my profile
	let req_my_profile = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "get_my_profile",
		}),
	);
	let result = req_my_profile.await?;
	result.print().await?;
	
	// -- Get other user profile
	let req_user_profile = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 2,
			"method": "get_user_profile",
			"params": {
				"id": "usr_demo1",
			}
		}),
	);
	let result = req_user_profile.await?;
	result.print().await?;

	// -- Check if user follows another user
	let req_is_following = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 3,
			"method": "is_following",
			"params": {
				"id": "usr_demo1",
			}
		}),
	);
	let result = req_is_following.await?;
	result.print().await?;
    
	// -- Get list of followers
	let req_list_followers = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 4,
			"method": "list_followers",
			"params": {
				"id": "usr_demo1",
			}
		}),
	);
	let result = req_list_followers.await?;
	result.print().await?;
    
	// -- Get list of followings
	let req_list_followings = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 5,
			"method": "list_followings",
			"params": {
				"id": "usr_demo1",
			}
		}),
	);
	let result = req_list_followings.await?;
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