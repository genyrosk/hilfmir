{ 
    id: MessageId(26),
    date: 2022-11-19T18:16:29Z,
    chat: Chat { 
        id: ChatId(887728385),
        kind: Private(ChatPrivate { 
            username: Some("genyrosk"),
            first_name: Some("Evgeny"),
            last_name: Some("Roskach"),
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None 
        }),
        photo: None,
        pinned_message: None,
        message_auto_delete_time: None 
    },
    via_bot: None,
    edit_date: None,
    media_kind: Text(MediaText { 
        text: "reply to this one msg",
        entities: [] 
    }),
    reply_markup: None,
    is_automatic_forward: false,
    has_protected_content: false,
    kind: Common(MessageCommon { 
        from: Some( User { 
            id: UserId(887728385),
            is_bot: false,
            first_name: "Evgeny",
            last_name: Some("Roskach"),
            username: Some("genyrosk"),
            language_code: Some("en"),
            is_premium: false,
            added_to_attachment_menu: false 
        }),
        sender_chat: None,
        author_signature: None,
        forward: None,
        reply_to_message: Some(Message {
            id: MessageId(22),
            date: 2022-11-19T18:06:41Z,
            chat: Chat { 
                id: ChatId(887728385),
                kind: Private(ChatPrivate { 
                    username: Some("genyrosk"),
                    first_name: Some("Evgeny"),
                    last_name: Some("Roskach"),
                    bio: None,
                    has_private_forwards: None,
                    has_restricted_voice_and_video_messages: None 
                }),
                photo: None,
                pinned_message: None,
                message_auto_delete_time: None 
            },
            via_bot: None,
            kind: Common(MessageCommon { 
                from: Some(User { 
                    id: UserId(887728385),
                    is_bot: false,
                    first_name: "Evgeny",
                    last_name: Some("Roskach"),
                    username: Some("genyrosk"),
                    language_code: Some("en"),
                    is_premium: false,
                    added_to_attachment_menu: false
                }),
                sender_chat: None,
                author_signature: None,
                forward: None,
                reply_to_message: None,
                edit_date: None,
                media_kind: Text(MediaText { 
                    text: "one more",
                    entities: [] 
                }),
                reply_markup: None,
                is_automatic_forward: false,
                has_protected_content: false 
            }) 
        }),
    }) 
}