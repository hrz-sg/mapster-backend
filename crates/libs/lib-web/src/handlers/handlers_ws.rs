use std::sync::Arc;
use axum::{
    extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}}, 
    response::IntoResponse
};
use futures_util::{SinkExt, StreamExt};
use lib_core::{ctx::Ctx, model::ModelManager, service::{Result, chat::ChatService}, ws::{notification::IntoWsNotification}};
use lib_rpc::{RpcRequest, RpcResources, router::RpcRouter};
use serde_json::json;
use tokio::sync::Mutex;
use crate::middleware::mw_auth::CtxW;

pub async fn handler(
    ws: WebSocketUpgrade,
    State((mm, rpc_router)): State<(ModelManager, Arc<RpcRouter>)>,
    ctx: CtxW,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        if let Err(err) = handle_socket(socket, mm, rpc_router, ctx.0).await {
            tracing::error!("websocket error: {:?}", err);
        }
    })
}

async fn handle_socket(
    ws: WebSocket,
    mm: ModelManager,
    rpc_router: Arc<RpcRouter>,
    ctx: Ctx,
) -> Result<()> {
    let user_id = ctx.user_id();

    // split into sender & receiver
    let (ws_tx, mut ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    // -- Get user's chats
    let chat_ids = ChatService::get_chats(&ctx, &mm).await?;

    // -- Subscribe to all chats
    let mut conn_ids = vec![]; // opened connections

    for chat_id in chat_ids {
        // -- Crate Receiver for add_socket
        let rx_for_socket = mm.ws().subscribe(&chat_id).await;
        let conn_id = mm.ws().add_socket(&user_id, rx_for_socket).await;
        conn_ids.push(conn_id);

        // create task Receiver
        let mut rx_for_task = mm.ws().subscribe(&chat_id).await;
        let ws_tx = ws_tx.clone();

        tokio::spawn(async move {
            while let Ok(msg) = rx_for_task.recv().await {
                let payload = msg.into_ws_notification().stringify().unwrap_or_default();
                
                if let Err(err) = ws_tx.lock().await.send(Message::Text(payload.into())).await {
                    tracing::error!("Failed to send WebSocket message for chat {}: {:?}", chat_id, err);
                    break;
                }
            }
        });
    }

    // -- Process RPC from client
    while let Some(Ok(msg)) = ws_rx.next().await {
        let Ok(text) = msg.to_text() else { continue };
        let Ok(req) = serde_json::from_str::<RpcRequest>(&text) else { continue };

        let ws_tx = ws_tx.clone();
        let rpc_router_clone = rpc_router.clone();
        let ctx_clone = ctx.clone();
        let mm_clone = mm.clone();
        let method = req.method;
        let id = req.id;
        let params = req.params;

        tokio::spawn(async move {
            let result = rpc_router_clone
                .call(&method, RpcResources { ctx: Some(ctx_clone), mm: mm_clone }, params)
                .await;

            let response = match result {
                Ok(v) => json!({ "jsonrpc": "2.0", "id": id, "result": v }),
                Err(e) => json!({ "jsonrpc": "2.0", "id": id, "error": e.to_string() }),
            };

            let _ = ws_tx.lock().await
                .send(Message::Text(response.to_string().into()))
                .await;
        });
    }

    // -- Auto unsubscription when close connection
    let ws_instance = mm.ws();
    for conn_id in conn_ids {
        ws_instance.remove_socket(&user_id, conn_id).await;
    }

    tracing::info!("WebSocket closed and unsubscribed user {}", user_id);

    Ok(())
}