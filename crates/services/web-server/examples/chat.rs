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
	let req_create_chat = hc.do_post(
		"/api/rpc",
		json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "create_chat",
			"params": {
				"data": {
					"chat_type": "Group",
					"title": "Test",
					"members": [
                        "usr_demo0", "usr_s-HX5PoKliFmPSJM4uVJa"
                    ]
				}
			}
		}),
	);
	let result = req_create_chat.await?;
	result.print().await?;

	// // -- Logout
	// let req_logout = hc.do_post(
	// 	"/api/logout",
	// 	json!({
	// 		"logout": true
	// 	}),
	// );
	// req_logout.await?.print().await?;

	Ok(())
}