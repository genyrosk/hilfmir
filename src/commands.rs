use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::{Auth, GoogleCloudClient};

#[derive(Debug, Clone)]
enum Language {
    English,
    German,
    French,
    Spanish,
    Russian,
    Korean,
}

impl Language {
    pub fn code(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
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
            Language::Spanish => "spanish",
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
            Language::Spanish => "🇪🇸",
            Language::Russian => "🇷🇺",
            Language::Korean => "🇰🇷",
        }
        .to_owned()
    }

    pub fn parse_code(code: &str) -> Option<Self> {
        let lang = match code {
            "en" => Some(Language::English),
            "de" => Some(Language::German),
            "fr" => Some(Language::French),
            "es" => Some(Language::Spanish),
            "ru" => Some(Language::Russian),
            "ko" => Some(Language::Korean),
            _ => None,
        };
        log::debug!("{} => {:?}", code, lang);
        lang
    }
}

fn parse_command_text(cmd_text: &str) -> (Option<Language>, Option<String>) {
    let maybe_code = &cmd_text
        .get(0..std::cmp::min(3, cmd_text.len()))
        .map(|s| s.trim());

    let lang = maybe_code.and_then(Language::parse_code);
    let text = cmd_text.trim().get(3..).map(|s| s.to_string());
    (lang, text)
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
            are supported: en, de, fr, es, ru, ko")]
    Translate(String),
    #[command(description = "shortcut for /translate.")]
    T(String),
}

pub async fn handle_command(
    bot: Bot,
    auth: Arc<Auth>,
    google_cloud_client: Arc<GoogleCloudClient>,
    msg: Message,
    cmd: Command,
) -> crate::Result<()> {
    log::info!(
        "new message from chat [{}] \"{}\": {:?}, cmd:",
        msg.chat.id,
        auth.get_chat_name(&msg.chat.id).unwrap_or_default(),
        cmd,
    );
    log::debug!("message json: {}", serde_json::json!(msg));

    let references_earlier_msg = msg.reply_to_message();
    let earlier_msg_text = references_earlier_msg
        .and_then(|msg| msg.text().map(|text| text.to_string()));
    log::info!("earlier_msg_text: {:?}", earlier_msg_text);

    let reply_to = msg.reply_to_message().unwrap_or(&msg);

    let bot_send_message = |text: String| async {
        bot.send_message(msg.chat.id, text)
            .reply_to_message_id(reply_to.id)
            .await
    };

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Translate(cmd_text) | Command::T(cmd_text) => {
            let cmd_text = cmd_text.trim();
            let (target, text) = match parse_command_text(cmd_text) {
                (Some(lang), text) => (lang, text),
                (None, _) => {
                    bot_send_message(
                        "Invalid target language.\nValid languages: \
                        en, de, fr, es, ru, ko"
                            .to_string(),
                    )
                    .await?;
                    return Ok(());
                }
            };

            let query_text = earlier_msg_text.or(text);
            log::info!(
                "target: {:?}, query_text: {:?}",
                target.name(),
                query_text
            );

            if query_text.is_none()
                || query_text.as_ref().map_or(0, |s| s.len()) == 0
            {
                bot_send_message(
                    "No text provided. Reply to a message \
                    or write text after the command \ne.g. `/t en some text`"
                        .to_string(),
                )
                .await?;
                return Ok(());
            }

            let query_text = query_text.unwrap(); //
            let tanslation = google_cloud_client
                .translate(&query_text, &target.code(), None)
                .await?;

            let detected_source_language = Language::parse_code(
                &tanslation.detected_source_language.unwrap_or_default(),
            );
            log::info!(
                "detected_source_language: {:?}, translation: {:?}",
                detected_source_language.as_ref().map(|lang| lang.name()),
                tanslation.translated_text
            );

            bot_send_message(format!(
                "{}➡️{}\n{}",
                detected_source_language
                    .map_or("".to_string(), |lang| lang.emoji()),
                target.emoji(),
                tanslation.translated_text
            ))
            .await?
        }
    };

    Ok(())
}
