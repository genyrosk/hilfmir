mod commands;
mod error;
mod translate;

pub use commands::{handle_command, Command};
pub use error::AppError;
pub use translate::GoogleCloudClient;

type Result<T> = std::result::Result<T, AppError>;
// type Result<T> = anyhow::Result<T>;
