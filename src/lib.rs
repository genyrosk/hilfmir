mod auth;
mod commands;
mod config;
mod error;
mod translate;
mod webhook;

pub use auth::Auth;
pub use commands::{handle_command, Command};
pub use config::{load_config, Config};
pub use error::AppError;
pub use translate::GoogleCloudClient;
pub use webhook::webhook;

type Result<T> = std::result::Result<T, AppError>;
