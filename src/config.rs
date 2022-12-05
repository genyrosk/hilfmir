use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use std::env::var;

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

#[derive(Deserialize, Debug, Clone)]
pub struct AllowedChat {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct EnvConfig {
    pub teloxide_token: Option<String>,
    pub google_cloud_api_key: Option<String>,
    pub allowed_chats: Vec<AllowedChat>,
    pub domain_host: String,
    pub bind_address: [u8; 4],
    pub port: u16,
    pub is_webhook_mode_enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct TomlConfig {
    pub teloxide_token: Option<String>,
    pub google_cloud_api_key: Option<String>,
    pub allowed_chats: Vec<AllowedChat>,
}

#[derive(Debug)]
pub struct Config {
    pub teloxide_token: SecretString,
    pub google_cloud_api_key: SecretString,
    pub allowed_chats: Vec<AllowedChat>,
    pub domain_host: String,
    pub bind_address: [u8; 4],
    pub port: u16,
    pub is_webhook_mode_enabled: bool,
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone)]
pub struct Webhook {
    pub path: String,
    pub url: String,
}

impl Config {
    pub fn new(
        teloxide_token: SecretString,
        google_cloud_api_key: SecretString,
        allowed_chats: Vec<AllowedChat>,
        domain_host: String,
        bind_address: [u8; 4],
        port: u16,
        is_webhook_mode_enabled: bool,
    ) -> Self {
        log::info!("Allowed Chat IDs: {:?}", allowed_chats);
        log::info!("Bind address port: {:?}", bind_address);
        log::info!("Service port: {}", port);
        log::info!("Webhook is enabled: {}", is_webhook_mode_enabled);

        let webhook = match is_webhook_mode_enabled {
            true => {
                let webhook_path = format!(
                    "/{}/api/v1/message",
                    teloxide_token.expose_secret()
                );
                let webhook_url =
                    format!("https://{}{}", domain_host, webhook_path);
                log::info!("Webhook url: {}", webhook_url);
                Some(Webhook {
                    path: webhook_path,
                    url: webhook_url,
                })
            }
            false => None,
        };

        Self {
            teloxide_token,
            google_cloud_api_key,
            allowed_chats,
            domain_host,
            bind_address,
            port,
            is_webhook_mode_enabled,
            webhook,
        }
    }
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

    Config::new(
        teloxide_token,
        google_cloud_api_key,
        allowed_chats,
        env_config.domain_host,
        env_config.bind_address,
        env_config.port,
        env_config.is_webhook_mode_enabled,
    )
}

pub fn get_config_from_env() -> EnvConfig {
    let allowed_chats = serde_json::from_str::<Vec<AllowedChat>>(
        &var("ALLOWED_CHATS")
            .ok()
            .unwrap_or_else(|| "[]".to_string()),
    )
    .expect("Bad format of ALLOWED_CHATS");

    let google_cloud_api_key = var("GOOGLE_CLOUD_API_KEY").ok();

    let teloxide_token = var("TELOXIDE_TOKEN").ok();

    let domain_host =
        var("DOMAIN_HOST").expect("DOMAIN_HOST env variable missing");

    let bind_address: [u8; 4] = var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .split('.')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<_>>()
        .try_into()
        .expect("BIND_HOST value has to be an of the form X.X.X.X (eg 0.0.0.0 or 127.0.0.1)");

    let port = var("BIND_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("BIND_PORT value has to be an integer");

    let is_webhook_mode_enabled: bool = var("WEBHOOK_MODE")
        .map(|val| val.to_lowercase())
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .expect(
            "Cannot convert WEBHOOK_MODE to bool. Applicable values are only \"true\" or \"false\"",
        );

    EnvConfig {
        teloxide_token,
        google_cloud_api_key,
        allowed_chats,
        domain_host,
        bind_address,
        port,
        is_webhook_mode_enabled,
    }
}

pub fn read_config_file() -> TomlConfig {
    let config_path = var(CONFIG_PATH_ENV).unwrap_or_else(|_| {
        log::warn!("CONFIG_PATH_ENV not set, assuming default config.toml");
        "./config.toml".to_string()
    });

    std::fs::read(&config_path)
        .map_err(|e| {
            log::warn!("Failed to read {config_path} Toml config file");
            e.to_string()
        })
        .ok()
        .map(|bytes| {
            toml::from_slice::<TomlConfig>(&bytes)
                .map_err(|e| e.to_string())
                .unwrap_or_else(|err| {
                    log::error!("failed to prase Toml config file: {err}");
                    std::process::exit(1);
                })
        })
        .unwrap_or_default()
}
