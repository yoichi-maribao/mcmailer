use std::sync::Mutex;

use tauri::{Emitter, Manager};

use mcmailer_lib::account::AccountStore;
use mcmailer_lib::commands::{self, AppState, SETTING_ACTIVE_ACCOUNT_EMAIL};
use mcmailer_lib::db::Database;
use mcmailer_lib::gmail_commands;
use mcmailer_lib::notification_commands;
use mcmailer_lib::notification_service::NotifiedMessages;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("mcmailer.db");
            let db = Database::open(&db_path)
                .map_err(|e| e.to_string())?;

            let accounts = db.load_all_accounts()
                .map_err(|e| e.to_string())?;
            let active_account_email = db.get_setting(SETTING_ACTIVE_ACCOUNT_EMAIL)
                .map_err(|e| e.to_string())?;

            let app_state = AppState {
                store: Mutex::new(AccountStore {
                    accounts,
                    active_account_email,
                }),
                db,
                notified_messages: NotifiedMessages::new(),
                pending_navigation: Mutex::new(None),
            };

            app.manage(app_state);

            let handle = app.handle().clone();
            tokio::spawn(mcmailer_lib::sse_client::start(handle));

            let handle = app.handle().clone();
            tokio::spawn(mcmailer_lib::polling::start(handle));

            let handle = app.handle().clone();
            tokio::spawn(mcmailer_lib::watch::start_renewal_loop(handle));

            Ok(())
        })
        .on_window_event(|window, event| {
            // When window gains focus, emit navigate-to-mail for pending notification
            if let tauri::WindowEvent::Focused(true) = event {
                let app_handle = window.app_handle();
                let state = app_handle.state::<AppState>();
                let payload = {
                    let mut pending = match state.pending_navigation.lock() {
                        Ok(p) => p,
                        Err(_) => return,
                    };
                    pending.take()
                };
                if let Some(nav) = payload {
                    let _ = window.set_focus();
                    let _ = app_handle.emit("navigate-to-mail", nav);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_oauth,
            commands::list_accounts,
            commands::switch_account,
            commands::remove_account,
            commands::get_active_account,
            commands::has_oauth_credentials,
            commands::set_oauth_credentials,
            gmail_commands::list_messages,
            gmail_commands::get_message,
            notification_commands::get_notification_settings,
            notification_commands::set_notification_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
