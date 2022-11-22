use std::sync::Arc;
use teloxide::prelude::*;

use hilfmir::{handle_command, load_config, Auth, Command, GoogleCloudClient};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Hilfmir bot...");

    let config = Arc::new(load_config());
    let auth = Arc::new(Auth::new(&config));

    let google_cloud_client =
        Arc::new(GoogleCloudClient::new(config.google_cloud_api_key.clone()));

    let bot = Bot::new(config.teloxide_token.expose_secret());

    let handler = Update::filter_message().branch(
        dptree::filter(|msg: Message, auth: Arc<Auth>| {
            auth.message_is_authorized(msg)
        })
        .filter_command::<Command>()
        .endpoint(handle_command),
    );

    Dispatcher::builder(bot, handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![config, auth, google_cloud_client.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
