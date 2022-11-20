use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::types::MessageKind;
use teloxide::utils::command::BotCommands;

pub struct Config {
    allowed_chat_ids: Vec<i64>,
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
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

fn authorize_chat(config: Arc<Config>, message: Message) -> bool {
    let chat_id = message.chat.id;
    config.allowed_chat_ids.contains(&chat_id.0)
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    log::debug!("handle_command => cmd: {:?}, msg: {:?}", cmd, msg);

    let references_earlier_msg = msg.reply_to_message();
    let input_text = references_earlier_msg.and_then(|msg| msg.text());

    let reply_to = msg.reply_to_message().unwrap_or(&msg);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Translate(cmd_text) | Command::T(cmd_text) => {
            let fut = match input_text {
                Some(input_text) => bot
                    .send_message(
                        msg.chat.id,
                        format!(
                            "Replying under message with id {} with text '{}'",
                            reply_to.id.0, input_text
                        ),
                    )
                    .reply_to_message_id(reply_to.id),
                None => bot
                    .send_message(
                        msg.chat.id,
                        format!(
                            "Replying to message with id {} and command text '{}'",
                            reply_to.id.0, cmd_text
                        ),
                    )
                    .reply_to_message_id(reply_to.id),
            };
            fut.await?
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let allowed_chat_ids = std::env::var("ALLOWED_CHAT_IDS")
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let env = Arc::new(Config { allowed_chat_ids });

    let bot = Bot::from_env();

    let handler = Update::filter_message().branch(
        dptree::filter(|msg: Message, config: Arc<Config>| authorize_chat(config, msg))
            .filter_command::<Command>()
            .endpoint(handle_command),
    );

    //
    //
    //
    let handler_old =
        Update::filter_message().endpoint(|bot: Bot, env: Arc<Config>, msg: Message| async move {
            log::debug!("msg: {:?}", msg);

            // ignore messages from unauthorized chats
            let chat_id = msg.chat.id;
            if !env.allowed_chat_ids.clone().contains(&chat_id.0) {
                return Ok(());
            }

            log::debug!("reply_to_message {:?}", msg.reply_to_message());

            let reply_to = msg.reply_to_message().unwrap_or(&msg);

            let input_text = reply_to.text();

            // if let Some(text) = msg.reply_to_message().and_then(|reply| reply.text()) {
            //     bot.send_message(
            //         msg.chat.id,
            //         format!("Replying under message with id {}", msg.id.0),
            //     )
            // }

            let fut = match (input_text) {
                Some(text) => bot
                    .send_message(
                        msg.chat.id,
                        format!(
                            "Replying under message with id {} and text '{}'",
                            msg.id.0, text
                        ),
                    )
                    .reply_to_message_id(reply_to.id),
                None => bot
                    .send_message(
                        msg.chat.id,
                        format!("Replying to message with id {} but no text", msg.id.0),
                    )
                    .reply_to_message_id(reply_to.id),
            };

            fut.await?;
            respond(())
        });

    Dispatcher::builder(bot, handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![env])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
