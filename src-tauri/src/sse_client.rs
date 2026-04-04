use std::time::Duration;

const BACKOFF_STEPS: [u64; 4] = [5, 10, 30, 60];

pub fn calculate_backoff(attempt: u32) -> Duration {
    let index = (attempt as usize).min(BACKOFF_STEPS.len() - 1);
    Duration::from_secs(BACKOFF_STEPS[index])
}

pub async fn start(app_handle: tauri::AppHandle) {
    use futures_util::StreamExt;
    use reqwest_eventsource::{Event, EventSource};
    use tauri::Manager;

    use crate::commands::AppState;

    let mut attempt: u32 = 0;

    loop {
        let state = app_handle.state::<AppState>();
        let settings = match crate::notification_commands::load_notification_settings(&state.db) {
            Ok(s) => s,
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        if !settings.enabled || settings.pubsub_server_url.is_empty() {
            tokio::time::sleep(Duration::from_secs(30)).await;
            continue;
        }

        let sse_url = format!("{}/events", settings.pubsub_server_url);
        let mut es = EventSource::get(&sse_url);

        loop {
            match es.next().await {
                Some(Ok(Event::Open)) => {
                    attempt = 0;
                    println!("[sse_client] Connected to SSE server");
                }
                Some(Ok(Event::Message(msg))) => {
                    attempt = 0;
                    if let Ok(notification) =
                        serde_json::from_str::<serde_json::Value>(&msg.data)
                    {
                        if let (Some(email), Some(history_id)) = (
                            notification["emailAddress"].as_str(),
                            notification["historyId"].as_str(),
                        ) {
                            let _ = crate::notification_service::process_notification(
                                email,
                                history_id,
                                &state,
                                &app_handle,
                            )
                            .await;
                        }
                    }
                }
                Some(Err(_)) | None => {
                    break;
                }
            }
        }

        let backoff = calculate_backoff(attempt);
        println!(
            "[sse_client] Reconnecting in {:?} (attempt {})",
            backoff, attempt
        );
        tokio::time::sleep(backoff).await;
        attempt = attempt.saturating_add(1);
    }
}
