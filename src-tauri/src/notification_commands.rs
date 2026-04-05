use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::AppState;
use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub enabled: bool,
    pub pubsub_subscription: String,
    pub pubsub_topic: String,
}

const SETTING_NOTIFICATIONS_ENABLED: &str = "notifications_enabled";
const SETTING_PUBSUB_SUBSCRIPTION: &str = "pubsub_subscription";
const SETTING_PUBSUB_TOPIC: &str = "pubsub_topic";

pub fn load_notification_settings(db: &Database) -> Result<NotificationSettings, String> {
    let enabled = db
        .get_setting(SETTING_NOTIFICATIONS_ENABLED)?
        .map(|v| v == "true")
        .unwrap_or(false);
    let pubsub_subscription = db
        .get_setting(SETTING_PUBSUB_SUBSCRIPTION)?
        .unwrap_or_default();
    let pubsub_topic = db.get_setting(SETTING_PUBSUB_TOPIC)?.unwrap_or_default();

    Ok(NotificationSettings {
        enabled,
        pubsub_subscription,
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
        SETTING_NOTIFICATIONS_ENABLED,
        if settings.enabled { "true" } else { "false" },
    )?;
    state
        .db
        .set_setting(SETTING_PUBSUB_SUBSCRIPTION, &settings.pubsub_subscription)?;
    state
        .db
        .set_setting(SETTING_PUBSUB_TOPIC, &settings.pubsub_topic)?;

    Ok(())
}
