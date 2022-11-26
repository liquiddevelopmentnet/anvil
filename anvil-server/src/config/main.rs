use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use paris::info;
use serde::Serialize;
use serde::Deserialize;

static mut CONFIG: Option<Configuration> = None;
const DEFAULT_CONFIG_NAME: &str = "anvil-config.yml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub bind_config: BindConfig,
    pub database_config: DatabaseConfig,
    pub plugins_enabled: bool,
    pub notification_config: NotificationConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BindConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub webhooks: Vec<WebhookConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebhookConfig {
    pub name: String,
    pub url: String,
    pub avatar_url: Option<String>,
    pub method: String,
    pub standard: WebhookStandard,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WebhookStandard {
    Discord,
    Slack,
    Generic,
}

impl Configuration {
    fn new() -> Self {
        Self {
            bind_config: BindConfig {
                address: "localhost".to_string(),
                port: 56971,
            },
            database_config: DatabaseConfig {
                address: "localhost".to_string(),
                port: 5432,
                username: "root".to_string(),
                password: "password".to_string(),
                database: "anvil".to_string(),
            },
            plugins_enabled: false,
            notification_config: NotificationConfig {
                enabled: true,
                webhooks: vec![
                    WebhookConfig {
                        name: "anvil Notifications".to_string(),
                        url: "https://discord.com/api/webhooks/1234567890/abcdefghijklmnopqrstuvwxyz".to_string(),
                        avatar_url: Some("https://example.com/yourimage.png".to_string()),
                        method: "POST".to_string(),
                        standard: WebhookStandard::Discord,
                    },
                ],
            },
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let yaml = serde_yaml::to_string(&self).unwrap();

        let mut file = std::fs::File::create(DEFAULT_CONFIG_NAME).unwrap();
        file.write_all(yaml.as_bytes()).unwrap();

        Ok(())
    }
}

pub fn load() {
    if Path::new(DEFAULT_CONFIG_NAME).exists() {
        let mut file = std::fs::File::open(DEFAULT_CONFIG_NAME).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Configuration = serde_yaml::from_str(&contents).unwrap();
        unsafe { CONFIG = Option::from(config); }
    } else {
        let new = Configuration::new();
        new.save().unwrap();
        info!("A {} file has been created in the current directory. Please configure it to your liking and restart the server.", DEFAULT_CONFIG_NAME);
        exit(0);
    }
}

pub fn get() -> Configuration {
    unsafe {
        CONFIG.as_ref().unwrap().clone()
    }
}