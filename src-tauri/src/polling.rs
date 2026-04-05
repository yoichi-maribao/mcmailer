use std::time::Duration;

use tauri::Manager;

use crate::commands::AppState;

pub async fn start(app_handle: tauri::AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let state = app_handle.state::<AppState>();

        let settings = match crate::notification_commands::load_notification_settings(&state.db) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if !settings.enabled {
            continue;
        }

        let watch_states = match state.db.load_all_watch_states() {
            Ok(states) => states,
            Err(_) => continue,
        };

        for (email, history_id, _expiration) in &watch_states {
            if let Err(e) = crate::notification_service::process_notification(
                email,
                history_id,
                &state,
                &app_handle,
            )
            .await
            {
                println!("[polling] Failed to process notification for {}: {}", email, e);
            }
        }
    }
}
