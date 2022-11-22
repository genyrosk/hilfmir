use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

const CONFIG_PATH_ENV: &str = "CONFIG_PATH";

#[derive(Debug, Deserialize, Clone)]
pub struct SecretString(Secret<String>);

impl SecretString {
    pub fn new(secret: String) -> Self {
        Self(Secret::new(secret))
    }
    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl Default for SecretString {
    fn default() -> Self {
        SecretString(Secret::new("".to_string()))
    }
}

impl From<String> for SecretString {
    fn from(secret: String) -> Self {
        Self::new(secret)
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ConfigOptions {
    pub teloxide_token: Option<String>,
    pub google_cloud_api_key: Option<String>,
    pub allowed_chats: Vec<AllowedChat>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub teloxide_token: SecretString,
    pub google_cloud_api_key: SecretString,
    pub allowed_chats: Vec<AllowedChat>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AllowedChat {
    pub id: i64,
    pub name: String,
}

pub fn load_config() -> Config {
    let toml_config = read_config_file();
    let env_config = get_config_from_env();

    let teloxide_token = env_config
        .teloxide_token
        .map(|t| {
            log::warn!("TELOXIDE_TOKEN is set in the environment");
            t
        }) // replace with `.inspect(...)` once it's stabilized
        .or(toml_config.teloxide_token)
        .expect("TELOXIDE_TOKEN not specified")
        .into();
    let google_cloud_api_key = env_config
        .google_cloud_api_key
        .map(|t| {
            log::warn!("GOOGLE_CLOUD_API_KEY is set in the environment");
            t
        })
        .or(toml_config.google_cloud_api_key)
        .expect("GOOGLE_CLOUD_API_KEY not specified")
        .into();
    let allowed_chats = match env_config.allowed_chats.is_empty() {
        false => env_config.allowed_chats,
        true => toml_config.allowed_chats,
    };

    if allowed_chats.is_empty() {
        log::warn!("No Chats are allowed to communicate with the bot");
    }
    log::info!("Allowed Chat IDs: {:?}", allowed_chats);

    Config {
        teloxide_token,
        google_cloud_api_key,
        allowed_chats,
    }
}

pub fn get_config_from_env() -> ConfigOptions {
    let allowed_chats = std::env::var("ALLOWED_CHAT_IDS")
        .ok()
        .map(|val| {
            val.split(',')
                .filter_map(|s| s.parse::<i64>().ok())
                .map(|id| AllowedChat {
                    id,
                    name: "".to_string(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let google_cloud_api_key = std::env::var("GOOGLE_CLOUD_API_KEY").ok();
    let teloxide_token = std::env::var("TELOXIDE_TOKEN").ok();

    ConfigOptions {
        teloxide_token,
        google_cloud_api_key,
        allowed_chats,
    }
}

pub fn read_config_file() -> ConfigOptions {
    let config_path = std::env::var(CONFIG_PATH_ENV).unwrap_or_else(|_| {
        log::warn!("CONFIG_PATH_ENV not set, assuming default config.toml");
        "./config.toml".to_string()
    });

    std::fs::read(config_path)
        .map_err(|e| e.to_string())
        .and_then(|bytes| {
            toml::from_slice::<ConfigOptions>(&bytes).map_err(|e| e.to_string())
        })
        .unwrap_or_else(|err| {
            log::error!("failed to read config: {err}");
            std::process::exit(1);
        })
}
