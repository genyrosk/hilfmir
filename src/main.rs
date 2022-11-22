use hilfmir::{handle_command, Command, GoogleCloudClient};
use std::sync::Arc;
use teloxide::prelude::*;

pub struct Config {
    allowed_chat_ids: Vec<i64>,
}

fn authorize_chat(config: Arc<Config>, message: Message) -> bool {
    let chat_id = message.chat.id;
    let is_authorized = config.allowed_chat_ids.contains(&chat_id.0);
    if !is_authorized {
        log::warn!("Chat [{}] is not authorized", &chat_id.0);
    }
    is_authorized
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Hilfmir bot...");

    let allowed_chat_ids = std::env::var("ALLOWED_CHAT_IDS")
        .expect("ALLOWED_CHAT_IDS not specified")
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    log::info!("Allowed Chat IDs: {:?}", allowed_chat_ids);
    let config = Arc::new(Config { allowed_chat_ids });

    let google_cloud_api_key = std::env::var("GOOGLE_CLOUD_API_KEY")
        .expect("GOOGLE_CLOUD_API_KEY not specified");
    let google_cloud_client =
        Arc::new(GoogleCloudClient::new(google_cloud_api_key));

    let bot = Bot::from_env();

    let handler = Update::filter_message().branch(
        dptree::filter(|msg: Message, config: Arc<Config>| {
            authorize_chat(config, msg)
        })
        .filter_command::<Command>()
        .endpoint(handle_command),
    );

    Dispatcher::builder(bot, handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![config, google_cloud_client.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
