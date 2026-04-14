// region: --- Imports

use lib_core::model::chat::{ChatMember, ChatMessage, ChatMessageFilter};
use lib_core::service::chat::{AddMemberPayload, ChatService, CreateChatPayload, EditMessagePayload, MarkAsReadPayload, RemoveMemberPayload, SendMessagePayload};
use lib_rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList, Result};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Imports

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_group_chat,
        get_or_create_direct_chat,
        // Json RPC over WS
        send_message,
        add_member,
        remove_member,
        list_messages,
        mark_as_read,
        list_chat_members,
        edit_message,
        delete_message,
        get_chats
    )
}

pub async fn create_group_chat(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CreateChatPayload>
) -> Result<String> {
    let ParamsForCreate { data } = params;

    let chat_id = ChatService::create_group_chat(&ctx, &mm, data).await?;

    Ok(chat_id)
}

pub async fn get_or_create_direct_chat(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<String> {
    let ParamsIded { id } = params;

    let chat_id = ChatService::get_or_create_direct_chat(&ctx, &mm, id).await?;

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

pub async fn add_member(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<AddMemberPayload>
) -> Result<()> {
    
    let ParamsForCreate { data } = params;
    
    let result = ChatService::add_member(&ctx, &mm, data).await?;

    Ok(result)
}

pub async fn remove_member(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<RemoveMemberPayload>
) -> Result<u64> {
    
    let ParamsForCreate { data } = params;
    
    let result = ChatService::remove_member(&ctx, &mm, data).await?;

    Ok(result)
}

pub async fn list_messages(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ChatMessageFilter>
) -> Result<Vec<ChatMessage>> {
    let ParamsList { filters, list_options } = params;
    
    let result = ChatService::list_messages(&ctx, &mm, filters, list_options).await?;

    Ok(result)
}

pub async fn mark_as_read(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<MarkAsReadPayload>
) -> Result<()> {
    let ParamsForUpdate { id, data } = params;
    
    let result = ChatService::mark_as_read(&ctx, &mm, id, data).await?;

    Ok(result)
}

pub async fn list_chat_members(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<Vec<ChatMember>> {
    let ParamsIded { id } = params;
    
    let result = ChatService::list_members(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn edit_message(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<EditMessagePayload>
) -> Result<ChatMessage> {
    let ParamsForUpdate { id, data } = params;
    
    let result = ChatService::edit_message(&ctx, &mm, id, data).await?;

    Ok(result)
}

pub async fn delete_message(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<()> {
    let ParamsIded { id } = params;
    
    let result = ChatService::delete_message(&ctx, &mm, id).await?;

    Ok(result)
}

pub async fn get_chats(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<Vec<String>> {
    
    let result = ChatService::get_chats(&ctx, &mm).await?;

    Ok(result)
}
