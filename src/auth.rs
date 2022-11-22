use std::collections::HashMap;
use teloxide::types::{ChatId, Message};

use crate::config::{AllowedChat, Config};

pub struct Auth {
    allowed_chats: HashMap<ChatId, AllowedChat>,
}

impl Auth {
    pub fn new(config: &Config) -> Self {
        Self {
            allowed_chats: config
                .allowed_chats
                .clone()
                .into_iter()
                .map(|chat| (ChatId(chat.id), chat))
                .collect(),
        }
    }

    pub fn message_is_authorized(&self, message: Message) -> bool {
        let chat_id = message.chat.id;
        let is_authorized = self.allowed_chats.contains_key(&chat_id);
        if !is_authorized {
            log::warn!("Chat [{}] is not authorized", &chat_id.0);
        }
        is_authorized
    }

    pub fn get_chat_name(&self, chat_id: &ChatId) -> Option<String> {
        self.allowed_chats
            .get(chat_id)
            .map(|chat| chat.name.clone())
    }
}
