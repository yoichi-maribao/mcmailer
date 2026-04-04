use std::sync::Mutex;

use tauri::Manager;

use mcmailer_lib::account::AccountStore;
use mcmailer_lib::commands::{self, AppState, SETTING_ACTIVE_ACCOUNT_EMAIL};
use mcmailer_lib::db::Database;
use mcmailer_lib::gmail_commands;

fn main() {
    tauri::Builder::default()
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
            };

            app.manage(app_state);
            Ok(())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
