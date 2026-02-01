use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::chat::{
    ChatBmc, ChatForCreate, ChatMessageBmc, ChatMessageForCreate, ChatParticipantBmc, ChatParticipantFilter,
    ChatParticipantForCreate, ChatParticipantForUpdate, ChatType, MessageType,
};
use crate::service::error::{Error, Result};

pub struct ChatService;

impl ChatService {
    pub async fn create_chat(
        ctx: &Ctx,
        mm: &ModelManager,
        chat_type: ChatType,
        title: Option<String>,
        participants: Vec<String>,
    ) -> Result<String> {
        // -- Create chat
        let chat_id = ChatBmc::create(ctx, mm, ChatForCreate { chat_type, title }).await?;

        // -- Add participants
        for user_id in participants {
            ChatParticipantBmc::create(
                ctx,
                mm,
                ChatParticipantForCreate {
                    chat_id: chat_id.clone(),
                    user_id,
                },
            )
            .await?;
        }

        Ok(chat_id)
    }

    /// -- Add user into chat
    pub async fn add_participant(ctx: &Ctx, mm: &ModelManager, chat_id: &str, user_id: &str) -> Result<()> {
        // -- Check if participant already in the chat
        let exists = ChatParticipantBmc::exists(
            ctx,
            mm,
            ChatParticipantFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
        )
        .await?;

        if exists {
            // -- If exists, then check if it's left
            let participants = ChatParticipantBmc::list(
                ctx,
                mm,
                Some(vec![ChatParticipantFilter {
                    chat_id: Some(chat_id.to_string()),
                    user_id: Some(user_id.to_string()),
                }]),
                None,
            )
            .await?;

            if let Some(participant) = participants.first() {
                if participant.left_at.is_some() {
                    // -- User left chat, recover him
                    ChatParticipantBmc::update_by_filter(
                        ctx,
                        mm,
                        ChatParticipantFilter {
                            chat_id: Some(chat_id.to_string()),
                            user_id: Some(user_id.to_string()),
                        },
                        ChatParticipantForUpdate { left_at: None },
                    )
                    .await?;
                }
            }
        } else {
            // -- Create participant if no records
            ChatParticipantBmc::create(
                ctx,
                mm,
                ChatParticipantForCreate {
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
        chat_id: &str,
        message_type: MessageType,
        text: Option<String>,
        post_id: Option<String>,
        journey_id: Option<String>,
        reply_to_id: Option<String>,
    ) -> Result<String> {
        let user_id = ctx.user_id(); // current user
        let is_participant = ChatParticipantBmc::exists(
            ctx,
            mm,
            ChatParticipantFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
        )
        .await?;

        if !is_participant {
            return Err(Error::permission_denied("User is not a participant of the chat"));
        }

        let chat_message_id = ChatMessageBmc::create(
            ctx,
            mm,
            ChatMessageForCreate {
                chat_id: chat_id.to_string(),
                message_type,
                text,
                post_id,
                journey_id,
                reply_to_id,
            },
        )
        .await?;

        Ok(chat_message_id)
    }

    /// -- Remove or leave the chat
    pub async fn remove_participant(ctx: &Ctx, mm: &ModelManager, chat_id: &str, user_id: &str) -> Result<u64> {
        // -- Update left_at
        let result = ChatParticipantBmc::update_by_filter(
            ctx,
            mm,
            ChatParticipantFilter {
                chat_id: Some(chat_id.to_string()),
                user_id: Some(user_id.to_string()),
            },
            ChatParticipantForUpdate {
                left_at: Some(chrono::Utc::now()),
            },
        )
        .await?;

        Ok(result)
    }
}
