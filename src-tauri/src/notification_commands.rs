use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::AppState;
use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub enabled: bool,
    pub pubsub_server_url: String,
    pub pubsub_topic: String,
}

pub fn load_notification_settings(db: &Database) -> Result<NotificationSettings, String> {
    let enabled = db
        .get_setting("notifications_enabled")?
        .map(|v| v == "true")
        .unwrap_or(false);
    let pubsub_server_url = db
        .get_setting("pubsub_server_url")?
        .unwrap_or_default();
    let pubsub_topic = db.get_setting("pubsub_topic")?.unwrap_or_default();

    Ok(NotificationSettings {
        enabled,
        pubsub_server_url,
        pubsub_topic,
    })
}

#[tauri::command]
pub async fn get_notification_settings(
    state: State<'_, AppState>,
) -> Result<NotificationSettings, String> {
    load_notification_settings(&state.db)
}

#[tauri::command]
pub async fn set_notification_settings(
    settings: NotificationSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.db.set_setting(
        "notifications_enabled",
        if settings.enabled { "true" } else { "false" },
    )?;
    state
        .db
        .set_setting("pubsub_server_url", &settings.pubsub_server_url)?;
    state
        .db
        .set_setting("pubsub_topic", &settings.pubsub_topic)?;

    Ok(())
}
