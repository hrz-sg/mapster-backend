use modql::filter::ListOptions;
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::chat::{
    ChatBmc, ChatForCreate, ChatMessageBmc, ChatMessageForCreate, ChatMemberBmc, ChatMemberFilter,
    ChatMemberForCreate, ChatMemberForUpdate, ChatType, MessageType,
};
use crate::service::error::{Error, Result};
use crate::ws::WsChatMessage;

pub struct ChatService;

impl ChatService {
    pub async fn create_chat(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreateChatPayload,
    ) -> Result<String> {

        let CreateChatPayload {
            chat_type,
            title,
            members,
        } = payload;

        // -- Create chat
        let chat_id = ChatBmc::create(ctx, mm, ChatForCreate { chat_type, title }).await?;

        // -- Add members
        for user_id in members {
            ChatMemberBmc::create(
                ctx,
                mm,
                ChatMemberForCreate {
                    chat_id: chat_id.clone(),
                    user_id,
                },
            )
            .await?;
        }

        Ok(chat_id)
    }

    /// -- Add user into chat
    pub async fn add_member(ctx: &Ctx, mm: &ModelManager, chat_id: &str, user_id: &str) -> Result<()> {
        // -- Check if member already in the chat
        let exists = ChatMemberBmc::exists(
            ctx,
            mm,
            ChatMemberFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
        )
        .await?;

        if exists {
            // -- If exists, then check if it's left
            let members = ChatMemberBmc::list(
                ctx,
                mm,
                Some(vec![ChatMemberFilter {
                    chat_id: Some(chat_id.to_string()),
                    user_id: Some(user_id.to_string()),
                }]),
                None,
            )
            .await?;

            if let Some(member) = members.first() {
                if member.left_at.is_some() {
                    // -- User left chat, recover him
                    ChatMemberBmc::update_by_filter(
                        ctx,
                        mm,
                        ChatMemberFilter {
                            chat_id: Some(chat_id.to_string()),
                            user_id: Some(user_id.to_string()),
                        },
                        ChatMemberForUpdate { left_at: None },
                    )
                    .await?;
                }
            }
        } else {
            // -- Create member if no records
            ChatMemberBmc::create(
                ctx,
                mm,
                ChatMemberForCreate {
                    chat_id: chat_id.to_string(),
                    user_id: user_id.to_string(),
                },
            )
            .await?;
        }

        Ok(())
    }

    /// -- Send message into chat
    pub async fn send_message(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: SendMessagePayload,
    ) -> Result<String> {

        let SendMessagePayload {
            chat_id,
            message_type,
            text,
            post_id,
            journey_id,
            reply_to_id,
        } = payload;

        let user_id = ctx.user_id(); // current user
        let is_member = ChatMemberBmc::exists(
            ctx,
            mm,
            ChatMemberFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
        )
        .await?;

        if !is_member {
            return Err(Error::permission_denied("User is not a member of the chat"));
        }

        let chat_message_id = ChatMessageBmc::create(
            ctx,
            mm,
            ChatMessageForCreate {
                chat_id: chat_id.to_string(),
                message_type,
                text: text.clone(),
                post_id,
                journey_id,
                reply_to_id,
            },
        )
        .await?;

        mm.ws().broadcast(
            &chat_id,
            WsChatMessage {
                chat_id: chat_id.to_string(),
                user_id: ctx.user_id().to_string(),
                text,
            }
        ).await;

        Ok(chat_message_id)
    }

    /// -- Remove or leave the chat
    pub async fn remove_member(ctx: &Ctx, mm: &ModelManager, chat_id: &str, user_id: &str) -> Result<u64> {
        // -- Update left_at
        let result = ChatMemberBmc::update_by_filter(
            ctx,
            mm,
            ChatMemberFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
            ChatMemberForUpdate {
                left_at: Some(chrono::Utc::now()),
            },
        )
        .await?;

        Ok(result)
    }

    /// -- Get user chats
    pub async fn get_chats(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<String>> {
        let members = ChatMemberBmc::list(
            ctx,
            mm,
            Some(vec![ChatMemberFilter {
                user_id: Some(ctx.user_id().to_string()),
                ..Default::default()
            }]),
            Some(ListOptions {
                order_bys: None,
                ..Default::default()
            }),
        )
        .await?;

        let chat_ids = members.into_iter().map(|m| m.chat_id).collect::<Vec<_>>();

        Ok(chat_ids)
    }
}


#[derive(Debug, Deserialize)]
pub struct CreateChatPayload {
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub members: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct SendMessagePayload {
    pub chat_id: String,
    pub message_type: MessageType,
    pub text: Option<String>,
    pub post_id: Option<String>,
    pub journey_id: Option<String>,
    pub reply_to_id: Option<String>,
}