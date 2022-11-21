use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::GoogleCloudClient;

enum Language {
    English,
    German,
    French,
    Russian,
    Korean,
}

impl Language {
    pub fn code(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
            Language::Russian => "ru",
            Language::Korean => "ko",
        }
        .to_owned()
    }

    pub fn name(&self) -> String {
        match self {
            Language::English => "english",
            Language::German => "german",
            Language::French => "french",
            Language::Russian => "russian",
            Language::Korean => "korean",
        }
        .to_owned()
    }

    pub fn emoji(&self) -> String {
        match self {
            Language::English => "🇬🇧",
            Language::German => "🇩🇪",
            Language::French => "🇫🇷",
            Language::Russian => "🇷🇺",
            Language::Korean => "🇰🇷",
        }
        .to_owned()
    }

    pub fn parse_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Language::English),
            "de" => Some(Language::German),
            "fr" => Some(Language::French),
            "ru" => Some(Language::Russian),
            "ko" => Some(Language::Korean),
            _ => None,
        }
    }
}

fn parse_command_text(cmd_text: &str) -> Option<(Language, String)> {
    let maybe_code = &cmd_text
        .get(0..std::cmp::min(3, cmd_text.len()))
        .map(|s| s.trim());
    log::debug!("maybe_code: {:?}", maybe_code);

    let opt = maybe_code
        .and_then(|code| Language::parse_code(code))
        .and_then(|lang| {
            cmd_text
                .trim()
                .get(3..)
                .map(|text| (lang, text.to_string()))
        });

    opt
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported with languages:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "translate to specified language e.g. \
            `/translate en Hallo Welt!`. You can also reply to messages. \
            Translations from any language into the following lanuages \
            are supported: en, de, fr, ru, ko ")]
    Translate(String),
    #[command(description = "shortcut for /translate.")]
    T(String),
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
            let (target, text) = match parse_command_text(cmd_text) {
                Some(ok) => ok,
                None => {
                    bot.send_message(
                        msg.chat.id,
                        "Invalid target language.\nValid languages: en, de, fr, ru, ko",
                    )
                    .reply_to_message_id(reply_to.id)
                    .await?;
                    return Ok(());
                }
            };

            let query_text = earlier_msg_text.unwrap_or(&text);
            log::debug!(
                "Translate or T command => cmd_text: {:?},  target: {}, query_text: {}",
                cmd_text,
                target.name(),
                query_text
            );

            if query_text.len() == 0 {
                bot.send_message(
                    msg.chat.id,
                    format!("➡️{}\nError: no text provided", target.emoji()),
                )
                .reply_to_message_id(reply_to.id)
                .await?;
                return Ok(());
            }

            let tanslation = google_cloud_client
                .translate(query_text, &target.code(), None)
                .await?;

            let detected_source_language = Language::parse_code(
                &tanslation
                    .detected_source_language
                    .unwrap_or("".to_string()),
            );

            bot.send_message(
                msg.chat.id,
                format!(
                    "{}➡️{}\n{}",
                    detected_source_language.map_or("".to_string(), |lang| lang.emoji()),
                    target.emoji(),
                    tanslation.translated_text
                ),
            )
            .reply_to_message_id(reply_to.id)
            .await?
        }
    };

    Ok(())
}
