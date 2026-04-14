// region: --- Imports

use std::collections::HashSet;

use modql::field::Fields;
use modql::filter::{ListOptions, OpValInt64, OpValValue, OpValsInt64, OpValsValue};
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::chat::{
    ChatBmc, ChatForCreate, ChatMember, ChatMemberBmc, ChatMemberFilter, ChatMemberForCreate, ChatMemberForUpdate, ChatMessage, ChatMessageBmc, ChatMessageFilter, ChatMessageForCreate, ChatMessageForUpdate, ChatRole, ChatType, MessageType
};
use crate::service::error::{Error, Result};
use crate::utils::generate_direct_chat_key;
use crate::ws::WsChatMessage;
// endregion: --- Imports
pub struct ChatService;

impl ChatService {
    pub async fn create_group_chat(
        ctx: &Ctx,
        mm: &ModelManager,
        payload: CreateChatPayload,
    ) -> Result<String> {

        let CreateChatPayload {
            title,
            members,
        } = payload;

        let creator_id = ctx.user_id().to_string();
        
        // -- deduplicate + include creator
        let mut members_set: HashSet<String> = members.into_iter().collect();
        members_set.insert(creator_id.clone());

        // -- limit
        const MAX_GROUP_SIZE: usize = 100;
        if members_set.len() > MAX_GROUP_SIZE {
            return Err(Error::PermissionDenied("Group size limit exceeded".into()));
        }

        // -- Create chat
        let chat_id = ChatBmc::create(
            ctx, 
            mm, 
            ChatForCreate {
                chat_type: ChatType::Group,
                title,
                direct_key: None,
            }
        ).await?;

        // -- Add creator as ADMIN
        ChatMemberBmc::create(
            ctx,
            mm,
            ChatMemberForCreate {
                chat_id: chat_id.clone(),
                user_id: creator_id.clone(),
                role: ChatRole::Admin,
            },
        ).await?;

        // -- Add others as MEMBER
        for user_id in members_set {
            if user_id == creator_id {
                continue;
            }

            ChatMemberBmc::create(
                ctx,
                mm,
                ChatMemberForCreate {
                    chat_id: chat_id.clone(),
                    user_id,
                    role: ChatRole::Member,
                },
            ).await?;
        }

        Ok(chat_id)
    }

    pub async fn get_or_create_direct_chat(
        ctx: &Ctx,
        mm: &ModelManager,
        other_user_id: String,
    ) -> Result<String> {

        let ctx_user_id = ctx.user_id();
        let key = generate_direct_chat_key(ctx_user_id, &other_user_id);

        // -- Create chat
        ChatBmc::create_on_conflict(
            ctx,
            mm,
            ChatForCreate {
                chat_type: ChatType::Direct,
                title: None,
                direct_key: Some(key.clone()),
            }
        ).await?;

        // -- Find chat
        let chat = ChatBmc::find_by_direct_key(ctx, mm, &key).await?;

        // -- Add users
        ChatMemberBmc::create(
            ctx, 
            mm,
        ChatMemberForCreate {
                chat_id: chat.id.clone(),
                user_id: ctx_user_id.to_owned(),
                role: ChatRole::Member,
            }
        ).await?;

        ChatMemberBmc::create(
            ctx,
            mm,
            ChatMemberForCreate {
                chat_id: chat.id.clone(),
                user_id: other_user_id.to_owned(),
                role: ChatRole::Member,
            }
        ).await?;

        Ok(chat.id)
    }

    /// -- Send message to chat
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
                chat_id: Some(chat_id.clone().into()),
                user_id: Some(user_id.into()),
                ..Default::default()
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

    /// -- Add user into chat
    pub async fn add_member(
        ctx: &Ctx, 
        mm: &ModelManager, 
        payload: AddMemberPayload,
    ) -> Result<()> {

        let AddMemberPayload {
            chat_id,
            user_id,
        } = payload;

        // -- Check if member already in the chat
        let exists = ChatMemberBmc::exists(
            ctx,
            mm,
            ChatMemberFilter {
                chat_id: Some(chat_id.clone().into()),
                user_id: Some(user_id.clone().into()),
                ..Default::default()
            },
        )
        .await?;

        if exists {
            // -- If exists, then check if it's left
            let members = ChatMemberBmc::list(
                ctx,
                mm,
                Some(vec![ChatMemberFilter {
                    chat_id: Some(chat_id.clone().into()),
                    user_id: Some(user_id.clone().into()),
                    ..Default::default()
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
                            chat_id: Some(chat_id.clone().into()),
                            user_id: Some(user_id.clone().into()),
                            ..Default::default()
                        },
                        ChatMemberForUpdate { ..Default::default() }, //TODO: need to improve
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
                    chat_id: chat_id.into(),
                    user_id: user_id.into(),
                    role: ChatRole::Member
                },
            )
            .await?;
        }

        Ok(())
    }

    /// -- Remove or leave the chat
    pub async fn remove_member(
        ctx: &Ctx, 
        mm: &ModelManager, 
        payload: RemoveMemberPayload,
    ) -> Result<u64> {

        let RemoveMemberPayload {
            chat_id,
            user_id,
        } = payload;

        // -- Update left_at
        let result = ChatMemberBmc::update_by_filter(
            ctx,
            mm,
            ChatMemberFilter {
                chat_id: Some(chat_id.into()),
                user_id: Some(user_id.into()),
                ..Default::default()
            },
            ChatMemberForUpdate {
                left_at: Some(chrono::Utc::now()),
                ..Default::default()
            },
        )
        .await?;

        Ok(result)
    }

    pub async fn list_messages(
        ctx: &Ctx, 
        mm: &ModelManager, 
        filters: Option<Vec<ChatMessageFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ChatMessage>> {
        
        // let messages_filter = Some(vec![ChatMessageFilter {
        //     chat_id: Some(chat_id.into()),
        //     seq: before_seq.map(|s| OpValsInt64(vec![OpValInt64::Lt(s)])),
        //     dtime: Some(OpValsValue(vec![
        //         OpValValue::Null(true)
        //     ])),
        //     ..Default::default()
        // }]);

        // let list_options = Some(ListOptions {
        //     limit: Some(30), // CAN BE CHANGED
        //     order_bys: Some("-seq".into()),
        //     ..Default::default()
        // });

        let mut messages = ChatMessageBmc::list(ctx, mm, filters, list_options).await?;

        messages.reverse();

        Ok(messages)
    }

    pub async fn mark_as_read(
        ctx: &Ctx, 
        mm: &ModelManager, 
        chat_id: String,
        payload: MarkAsReadPayload,
    ) -> Result<()> {

        let MarkAsReadPayload {
            seq,
        } = payload;

        ChatMemberBmc::update_seq(
            ctx,
            mm,
            &chat_id,
            seq,
        ).await?;

        Ok(())
    }

    pub async fn list_members(
        ctx: &Ctx, 
        mm: &ModelManager, 
        chat_id: String,
    ) -> Result<Vec<ChatMember>> {
        
        let filter = Some(vec![ChatMemberFilter {
            chat_id: Some(chat_id.into()),
            ..Default::default()
        }]);

        let members = ChatMemberBmc::list(ctx, mm, filter, None).await?;

        Ok(members)
    }

    pub async fn sync_messages(
        ctx: &Ctx, 
        mm: &ModelManager, 
        chat_id: String,
        seq: i64,
    ) -> Result<Vec<ChatMessage>> {

        let filter = Some(vec![ChatMessageFilter {
            chat_id: Some(chat_id.into()),
            seq: Some(OpValsInt64(vec![
                OpValInt64::Gt(seq)
            ])),
            dtime: Some(OpValsValue(vec![
                OpValValue::Null(true)
            ])),
            ..Default::default()
        }]);

        let list_options = Some(ListOptions {
            order_bys: Some("seq".into()), // ASC
            ..Default::default()
        });

        let messages = ChatMessageBmc::list(ctx, mm, filter, list_options).await?;
        
        Ok(messages)
    }

    pub async fn edit_message(
        ctx: &Ctx, 
        mm: &ModelManager, 
        message_id: String,
        edit_message_p: EditMessagePayload,
    ) -> Result<ChatMessage> {

        let EditMessagePayload {
            text,
        } = edit_message_p;

        // -- Get msg
        let message = ChatMessageBmc::get(ctx, mm, &message_id).await?;

        // -- Check the owner
        if message.user_id != ctx.user_id() {
            return Err(Error::PermissionDenied("Can not edit not your own message".into()))
        }

        // -- Update message
        ChatMessageBmc::update(
            ctx,
            mm,
            &message_id,
            ChatMessageForUpdate {
                text,
                mtime: Some(chrono::Utc::now()),
                ..Default::default()
            },
        ).await?;

        // -- Get updated comment
        let updated_comment = ChatMessageBmc::get(ctx, mm, &message_id).await?;
        
        Ok(updated_comment)
    }

    pub async fn delete_message(
        ctx: &Ctx, 
        mm: &ModelManager, 
        message_id: String,
    ) -> Result<()> {

        // -- Get message
        let message = ChatMessageBmc::get(ctx, mm, &message_id).await?;

        if message.user_id != ctx.user_id() {
            return Err(Error::PermissionDenied("You can not delete not your message".into()));
        }

        // -- Soft delete
        ChatMessageBmc::update(
            ctx,
            mm,
            &message_id,
            ChatMessageForUpdate {
                dtime: Some(chrono::Utc::now()),
                ..Default::default()
            },
        ).await?;

        Ok(())
    }

    pub async fn get_chats(
        ctx: &Ctx,
        mm: &ModelManager
    ) -> Result<Vec<String>> {
        let members = ChatMemberBmc::list(
            ctx,
            mm,
            Some(vec![ChatMemberFilter {
                user_id: Some(ctx.user_id().into()),
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
    pub title: Option<String>,
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberPayload {
    pub chat_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoveMemberPayload {
    pub chat_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesPayload {
    pub chat_id: String,
    pub before_seq: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct EditMessagePayload {
    pub text: Option<String>,
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

#[derive(Fields, Default, Deserialize)]
pub struct MarkAsReadPayload {
    pub seq: i64,
}