mod auth;
mod commands;
mod config;
mod error;
mod translate;

pub use auth::Auth;
pub use commands::{handle_command, Command};
pub use config::{load_config, Config};
pub use error::AppError;
pub use translate::GoogleCloudClient;

type Result<T> = std::result::Result<T, AppError>;
