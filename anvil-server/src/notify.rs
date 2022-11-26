use std::fmt::format;
use std::future::Future;
use paris::{info};
use reqwest::Client;
use std::option::Option;
use crate::config;
use crate::config::{WebhookConfig, WebhookStandard};

static mut WEBHOOKS: Option<Vec<WebhookConfig>> = None;

pub struct CustomField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

pub struct Author {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

pub fn init() {
    unsafe {
        WEBHOOKS = Option::from(config::get().notification_config.webhooks.clone());
    }
}

pub fn get_webhooks() -> Vec<WebhookConfig> {
    unsafe {
        WEBHOOKS.as_ref().unwrap().clone()
    }
}

pub async fn send_notification(
    title: &str,
    description: &str,
    custom_fields: &Option<Vec<CustomField>>,
    author: &Option<Author>,
) {
    for webhook in get_webhooks() {
        match webhook.standard {
            WebhookStandard::Discord => {
                send_discord_notification(
                    webhook,
                    title,
                    description,
                    custom_fields,
                    author,
                ).await.expect("Failed to send discord notification");
            }
            WebhookStandard::Slack => {
                todo!("Slack webhook support is not yet implemented!");
            }
            WebhookStandard::Generic => {
                todo!("Generic webhook support is not yet implemented!");
            }
        }
    }
}

async fn send_discord_notification(
    webhook: WebhookConfig,
    title: &str,
    description: &str,
    custom_fields: &Option<Vec<CustomField>>,
    author: &Option<Author>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut fields = Vec::new();

    if custom_fields.is_some() {
        for field in custom_fields.as_ref().unwrap() {
            fields.push(serde_json::json!({
                "name": field.name,
                "value": field.value,
                "inline": field.inline,
            }));
        }
    }

    let mut author_json = serde_json::json!({});

    if author.is_some() {
        author_json = serde_json::json!({
            "name": author.as_ref().unwrap().name,
            "url": author.as_ref().unwrap().url,
            "icon_url": author.as_ref().unwrap().icon_url,
        });
    }

    let desc = if description.is_empty() {
        description.to_string()
    } else {
        format!("```{}```", description)
    };

    let body = serde_json::json!({
        "username": webhook.name,
        "avatar_url": webhook.avatar_url.unwrap_or_else(|| String::from("")),
        "embeds": [
            {
                "title": title,
                "description": desc,
                "author": author_json,
                "fields": fields,
            }
        ]
    });
    let res = client.post(&webhook.url)
        .body(body.to_string())
        .header("Content-Type", "application/json")
        .header("User-Agent", format!("Anvil/{}/{}", &env!("VERGEN_GIT_SHA")[..7], &env!("VERGEN_GIT_BRANCH")))
        .send()
        .await?;
    Ok(())
}