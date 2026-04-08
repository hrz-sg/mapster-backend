// region: --- Modules

use lib_core::service::chat::{ChatService, CreateChatPayload, SendMessagePayload};
use lib_rpc::{ParamsForCreate, Result};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_chat,
        send_message,
    )
}

pub async fn create_chat(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreateChatPayload>
) -> Result<String> {
    let ParamsForCreate { data } = params;

    let chat_id = ChatService::create_chat(&ctx, &mm, data).await?;

    Ok(chat_id)
}

pub async fn send_message(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<SendMessagePayload>
) -> Result<String> {
    
    let ParamsForCreate { data } = params;
    
    let result = ChatService::send_message(&ctx, &mm, data).await?;

    Ok(result)
}
