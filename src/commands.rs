use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::GoogleCloudClient;

enum TranslationMap {
    EnglishToGerman,
    GermanToEnglish,
}

enum Language {
    English,
    German,
}

impl Language {
    fn code(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
        }
        .to_owned()
    }
    fn name(&self) -> String {
        match self {
            Language::English => "english",
            Language::German => "german",
        }
        .to_owned()
    }
    fn emoji(&self) -> String {
        match self {
            Language::English => "🇬🇧",
            Language::German => "🇩🇪",
        }
        .to_owned()
    }
}

impl TranslationMap {
    pub fn source(&self) -> Language {
        match self {
            Self::EnglishToGerman => Language::English,
            Self::GermanToEnglish => Language::German,
        }
    }

    pub fn target(&self) -> Language {
        match self {
            Self::EnglishToGerman => Language::German,
            Self::GermanToEnglish => Language::English,
        }
    }
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "translate replied-to message")]
    Translate(String),
    #[command(description = "translate replied-to message")]
    T(String),
    #[command(description = "demo: handle a username.")]
    Username(String),
    #[command(
        description = "demo: handle a username and an age.",
        parse_with = "split"
    )]
    UsernameAndAge { username: String, age: u8 },
}

pub async fn handle_command(
    bot: Bot,
    google_cloud_client: Arc<GoogleCloudClient>,
    msg: Message,
    cmd: Command,
) -> crate::Result<()> {
    log::debug!("");
    log::debug!("");
    log::debug!("");
    log::debug!("handle_command => cmd: {:?}, msg:", cmd,);
    log::debug!("{}", serde_json::json!(msg));

    let references_earlier_msg = msg.reply_to_message();
    let earlier_msg_text = references_earlier_msg.and_then(|msg| msg.text());
    log::debug!("earlier_msg_text: {:?}", earlier_msg_text);

    let reply_to = msg.reply_to_message().unwrap_or(&msg);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Translate(cmd_text) | Command::T(cmd_text) => {
            log::debug!("cmd_text: {:?}", cmd_text);
            let cmd_text = cmd_text.trim();

            let lang_param = &cmd_text
                .get(0..std::cmp::min(3, cmd_text.len()))
                .map(|s| s.trim());
            log::debug!("lang_param: {:?}", lang_param);

            let (lang_map, cmd_text) = match lang_param {
                Some("de") => (
                    TranslationMap::EnglishToGerman,
                    cmd_text.trim().get(3..).unwrap_or(""),
                ),
                Some("en") => (
                    TranslationMap::GermanToEnglish,
                    cmd_text.trim().get(3..).unwrap_or(""),
                ),
                _ => (
                    TranslationMap::GermanToEnglish,
                    cmd_text.get(..).unwrap_or(""),
                ),
            };
            let source = lang_map.source();
            let target = lang_map.target();

            let query_text = earlier_msg_text.unwrap_or(&cmd_text);
            log::debug!(
                "Translate or T command => cmd_text: {:?}, source: {}, target: {}, query_text: {}",
                cmd_text,
                source.name(),
                target.name(),
                query_text
            );

            if query_text.len() == 0 {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "{}➡️{}\nError: no text provided",
                        source.emoji(),
                        target.emoji()
                    ),
                )
                .reply_to_message_id(reply_to.id)
                .await?;
                return Ok(());
            }

            let tanslation = google_cloud_client
                .translate(query_text, &source.code(), &target.code())
                .await?;
            bot.send_message(
                msg.chat.id,
                format!("{}➡️{}\n{}", source.emoji(), target.emoji(), tanslation),
            )
            .reply_to_message_id(reply_to.id)
            .await?

            // match input_text {
            //     Some(input_text) => {
            //         let tanslation = google_cloud_client
            //             .translate(input_text, "de", "en")
            //             .await?;
            //         bot.send_message(
            //             msg.chat.id,
            //             format!(
            //                 "Replying under message with id {} \ntext: {}\ntranslation: {}",
            //                 reply_to.id.0, input_text, tanslation
            //             ),
            //         )
            //         .reply_to_message_id(reply_to.id)
            //         .await?
            //     }
            //     None => {
            //         bot.send_message(
            //             msg.chat.id,
            //             format!(
            //                 "Replying to message with id {} \ntext: {}\ntranslation: {}",
            //                 reply_to.id.0, cmd_text
            //             ),
            //         )
            //         .reply_to_message_id(reply_to.id)
            //         .await?
            //     }
            // }
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}
