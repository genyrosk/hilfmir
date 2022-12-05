use axum::response::IntoResponse;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use teloxide::dispatching::update_listeners::StatefulListener;
use teloxide::prelude::*;
use teloxide::stop::{mk_stop_token, StopToken};

use crate::Config;

// Original implementation:
// https://github.com/zamazan4ik/npaperbot-telegram/blob/2eb14ec7121153768deb0d3763451ae58fa78572/src/webhook.rs

async fn telegram_request(
    input: String,
    tx: axum::extract::Extension<
        tokio::sync::mpsc::UnboundedSender<
            Result<teloxide::types::Update, String>,
        >,
    >,
) -> impl IntoResponse {
    log::info!("Webhook input: {}", input);
    let try_parse = match serde_json::from_str(&input) {
        Ok(update) => Ok(update),
        Err(error) => {
            log::error!(
                "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                       This is a bug in teloxide, please open an issue here: \
                       https://github.com/teloxide/teloxide/issues.",
                error,
                input
            );
            Err(error)
        }
    };
    if let Ok(update) = try_parse {
        tx.send(Ok(update))
            .expect("Cannot send an incoming update from the webhook")
    }

    axum::http::StatusCode::OK
}

pub async fn webhook(
    config: Arc<Config>,
    bot: Bot,
) -> impl teloxide::dispatching::update_listeners::UpdateListener<Err = String>
{
    let webhook = &config
        .webhook
        .clone()
        .expect("Webhook config is expected when setting WEBHOOK_MODE=true");

    bot.set_webhook(webhook.url.parse().unwrap())
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let app = axum::Router::new()
        .route("/", get(root))
        .route(webhook.path.as_str(), axum::routing::post(telegram_request))
        .layer(
            tower::ServiceBuilder::new()
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .layer(tower_http::add_extension::AddExtensionLayer::new(tx))
                .into_inner(),
        );

    let server_address = SocketAddr::from((config.bind_address, config.port));
    tracing::debug!("server listening on {}", server_address);

    tokio::spawn(async move {
        axum::Server::bind(&server_address)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .expect("Axum server error")
    });

    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    fn streamf<S, T>(state: &mut (S, T)) -> &mut S {
        &mut state.0
    }

    let (stop_token, _stop_flag) = mk_stop_token();
    StatefulListener::new(
        (stream, stop_token),
        streamf,
        |state: &mut (_, StopToken)| state.1.clone(),
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

async fn root() -> &'static str {
    "ok"
}
