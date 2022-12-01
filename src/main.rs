use std::sync::Arc;
use teloxide::prelude::*;

use hilfmir::webhook;
use hilfmir::{handle_command, load_config, Auth, Command, GoogleCloudClient};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init(); // using pretty_env_logger instead for now
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

    let mut bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![
            config.clone(),
            auth,
            google_cloud_client.clone()
        ])
        .enable_ctrlc_handler()
        .build();

    if config.is_webhook_mode_enabled {
        log::info!("Webhook mode activated");
        let rx = webhook(config, bot);
        bot_dispatcher
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text(
                    "An error from the update listener",
                ),
            )
            .await;
    } else {
        log::info!("Long polling mode activated");
        bot_dispatcher.dispatch().await;
    }
}
