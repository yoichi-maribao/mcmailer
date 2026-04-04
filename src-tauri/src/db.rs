use std::path::Path;
use std::sync::Mutex;

use rusqlite::OptionalExtension;
use rusqlite::Connection;

use crate::account::Account;

const CREATE_ACCOUNTS_TABLE: &str = "\
    CREATE TABLE IF NOT EXISTS accounts (\
        email TEXT PRIMARY KEY,\
        access_token TEXT NOT NULL,\
        refresh_token TEXT NOT NULL,\
        expires_at INTEGER NOT NULL\
    )";

const CREATE_SETTINGS_TABLE: &str = "\
    CREATE TABLE IF NOT EXISTS settings (\
        key TEXT PRIMARY KEY,\
        value TEXT NOT NULL\
    )";

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, String> {
        let conn = if path == Path::new(":memory:") {
            Connection::open_in_memory()
        } else {
            Connection::open(path)
        }
        .map_err(|e| format!("Failed to open database: {}", e))?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| format!("Failed to set journal mode: {}", e))?;

        conn.execute(CREATE_ACCOUNTS_TABLE, [])
            .map_err(|e| format!("Failed to create accounts table: {}", e))?;

        conn.execute(CREATE_SETTINGS_TABLE, [])
            .map_err(|e| format!("Failed to create settings table: {}", e))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn upsert_account(&self, account: &Account) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO accounts (email, access_token, refresh_token, expires_at) \
             VALUES (?1, ?2, ?3, ?4)",
            (
                &account.email,
                &account.access_token,
                &account.refresh_token,
                account.expires_at,
            ),
        )
        .map_err(|e| format!("Failed to upsert account: {}", e))?;

        Ok(())
    }

    pub fn load_all_accounts(&self) -> Result<Vec<Account>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT email, access_token, refresh_token, expires_at FROM accounts")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    email: row.get(0)?,
                    access_token: row.get(1)?,
                    refresh_token: row.get(2)?,
                    expires_at: row.get(3)?,
                })
            })
            .map_err(|e| format!("Failed to query accounts: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to read account row: {}", e))?;

        Ok(accounts)
    }

    pub fn delete_account(&self, email: &str) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute("DELETE FROM accounts WHERE email = ?1", [email])
            .map_err(|e| format!("Failed to delete account: {}", e))?;

        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT value FROM settings WHERE key = ?1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let result = stmt
            .query_row([key], |row| row.get(0))
            .optional()
            .map_err(|e| format!("Failed to get setting: {}", e))?;

        Ok(result)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )
        .map_err(|e| format!("Failed to set setting: {}", e))?;

        Ok(())
    }

    pub fn delete_setting(&self, key: &str) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute("DELETE FROM settings WHERE key = ?1", [key])
            .map_err(|e| format!("Failed to delete setting: {}", e))?;

        Ok(())
    }
}
