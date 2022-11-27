use serde::{Serialize};
use serde::Deserialize;
use crate::*;
use crate::config::{save_config, YamlConfig};

static mut CONFIG: Option<MainConfiguration> = None;

impl YamlConfig for MainConfiguration {
    const DEFAULT_CONFIG_NAME: &'static str = "anvil-config.yml";

    fn new() -> Self {
        MainConfiguration {
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

    fn save(&self) -> Result<(), io::Error> {
        save_config(self, Self::DEFAULT_CONFIG_NAME)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MainConfiguration {
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

pub fn load() -> bool {
    unsafe {
        let load = config::load_config(MainConfiguration::DEFAULT_CONFIG_NAME);
        if load.is_some() {
            CONFIG = load;
            false
        } else {
            true
        }
    }
}

pub fn get() -> MainConfiguration {
    unsafe {
        CONFIG.as_ref().unwrap().clone()
    }
}