use mcmailer_lib::account::Account;
use mcmailer_lib::db::Database;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_account(email: &str) -> Account {
        Account {
            email: email.to_string(),
            access_token: format!("access_{}", email),
            refresh_token: format!("refresh_{}", email),
            expires_at: 1700000000,
        }
    }

    fn open_in_memory_db() -> Database {
        Database::open(Path::new(":memory:")).expect("Failed to open in-memory database")
    }

    // --- Database::open ---

    #[test]
    fn should_open_in_memory_database() {
        // Given: an in-memory path
        // When: opening the database
        let result = Database::open(Path::new(":memory:"));

        // Then: database opens successfully
        assert!(result.is_ok());
    }

    #[test]
    fn should_open_database_at_file_path() {
        // Given: a temporary file path
        let dir = tempfile::tempdir().expect("Failed to create temp dir");
        let db_path = dir.path().join("test.db");

        // When: opening the database at that path
        let result = Database::open(&db_path);

        // Then: database opens successfully and file is created
        assert!(result.is_ok());
        assert!(db_path.exists());
    }

    #[test]
    fn should_return_error_when_path_is_invalid() {
        // Given: a path in a non-existent directory
        let invalid_path = Path::new("/nonexistent/directory/that/does/not/exist/test.db");

        // When: opening the database
        let result = Database::open(invalid_path);

        // Then: returns an error
        assert!(result.is_err());
    }

    // --- upsert_account ---

    #[test]
    fn should_insert_new_account() {
        // Given: an empty database and a new account
        let db = open_in_memory_db();
        let account = create_test_account("user1@gmail.com");

        // When: upserting the account
        let result = db.upsert_account(&account);

        // Then: succeeds and account can be loaded
        assert!(result.is_ok());
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].email, "user1@gmail.com");
        assert_eq!(accounts[0].access_token, "access_user1@gmail.com");
        assert_eq!(accounts[0].refresh_token, "refresh_user1@gmail.com");
        assert_eq!(accounts[0].expires_at, 1700000000);
    }

    #[test]
    fn should_update_existing_account_on_upsert() {
        // Given: a database with an existing account
        let db = open_in_memory_db();
        let account = create_test_account("user1@gmail.com");
        db.upsert_account(&account).unwrap();

        // When: upserting the same email with updated tokens
        let updated = Account {
            email: "user1@gmail.com".to_string(),
            access_token: "new_access_token".to_string(),
            refresh_token: "new_refresh_token".to_string(),
            expires_at: 1800000000,
        };
        let result = db.upsert_account(&updated);

        // Then: only one account exists with updated values
        assert!(result.is_ok());
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].access_token, "new_access_token");
        assert_eq!(accounts[0].refresh_token, "new_refresh_token");
        assert_eq!(accounts[0].expires_at, 1800000000);
    }

    #[test]
    fn should_insert_multiple_accounts() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: inserting multiple accounts
        db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user2@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user3@gmail.com")).unwrap();

        // Then: all accounts are stored
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 3);
    }

    // --- load_all_accounts ---

    #[test]
    fn should_return_empty_vec_when_no_accounts() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: loading all accounts
        let accounts = db.load_all_accounts().unwrap();

        // Then: returns empty vec
        assert!(accounts.is_empty());
    }

    #[test]
    fn should_load_all_stored_accounts_with_correct_fields() {
        // Given: a database with two accounts
        let db = open_in_memory_db();
        let account1 = Account {
            email: "alice@gmail.com".to_string(),
            access_token: "access_alice".to_string(),
            refresh_token: "refresh_alice".to_string(),
            expires_at: 1700000000,
        };
        let account2 = Account {
            email: "bob@gmail.com".to_string(),
            access_token: "access_bob".to_string(),
            refresh_token: "refresh_bob".to_string(),
            expires_at: 1800000000,
        };
        db.upsert_account(&account1).unwrap();
        db.upsert_account(&account2).unwrap();

        // When: loading all accounts
        let accounts = db.load_all_accounts().unwrap();

        // Then: all fields are preserved correctly
        assert_eq!(accounts.len(), 2);
        let emails: Vec<&str> = accounts.iter().map(|a| a.email.as_str()).collect();
        assert!(emails.contains(&"alice@gmail.com"));
        assert!(emails.contains(&"bob@gmail.com"));

        let alice = accounts.iter().find(|a| a.email == "alice@gmail.com").unwrap();
        assert_eq!(alice.access_token, "access_alice");
        assert_eq!(alice.refresh_token, "refresh_alice");
        assert_eq!(alice.expires_at, 1700000000);

        let bob = accounts.iter().find(|a| a.email == "bob@gmail.com").unwrap();
        assert_eq!(bob.access_token, "access_bob");
        assert_eq!(bob.refresh_token, "refresh_bob");
        assert_eq!(bob.expires_at, 1800000000);
    }

    // --- delete_account ---

    #[test]
    fn should_delete_existing_account() {
        // Given: a database with two accounts
        let db = open_in_memory_db();
        db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user2@gmail.com")).unwrap();

        // When: deleting the first account
        let result = db.delete_account("user1@gmail.com");

        // Then: only the second account remains
        assert!(result.is_ok());
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].email, "user2@gmail.com");
    }

    #[test]
    fn should_succeed_when_deleting_nonexistent_account() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: deleting an account that doesn't exist
        let result = db.delete_account("nonexistent@gmail.com");

        // Then: succeeds without error (DELETE WHERE is idempotent)
        assert!(result.is_ok());
    }

    #[test]
    fn should_not_affect_other_accounts_when_deleting() {
        // Given: a database with three accounts
        let db = open_in_memory_db();
        db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user2@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user3@gmail.com")).unwrap();

        // When: deleting the middle account
        db.delete_account("user2@gmail.com").unwrap();

        // Then: other accounts are untouched
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 2);
        let emails: Vec<&str> = accounts.iter().map(|a| a.email.as_str()).collect();
        assert!(emails.contains(&"user1@gmail.com"));
        assert!(emails.contains(&"user3@gmail.com"));
    }

    // --- set_setting / get_setting ---

    #[test]
    fn should_set_and_get_setting() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: setting a value
        db.set_setting("client_id", "my_client_id_123").unwrap();

        // Then: the value can be retrieved
        let value = db.get_setting("client_id").unwrap();
        assert_eq!(value, Some("my_client_id_123".to_string()));
    }

    #[test]
    fn should_return_none_for_nonexistent_setting() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: getting a setting that doesn't exist
        let value = db.get_setting("nonexistent_key").unwrap();

        // Then: returns None
        assert!(value.is_none());
    }

    #[test]
    fn should_overwrite_existing_setting() {
        // Given: a database with an existing setting
        let db = open_in_memory_db();
        db.set_setting("client_id", "old_value").unwrap();

        // When: setting the same key with a new value
        db.set_setting("client_id", "new_value").unwrap();

        // Then: the new value is returned
        let value = db.get_setting("client_id").unwrap();
        assert_eq!(value, Some("new_value".to_string()));
    }

    #[test]
    fn should_store_multiple_independent_settings() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: setting multiple keys
        db.set_setting("client_id", "id_123").unwrap();
        db.set_setting("client_secret", "secret_456").unwrap();
        db.set_setting("active_account_email", "user@gmail.com").unwrap();

        // Then: each key returns its own value
        assert_eq!(
            db.get_setting("client_id").unwrap(),
            Some("id_123".to_string())
        );
        assert_eq!(
            db.get_setting("client_secret").unwrap(),
            Some("secret_456".to_string())
        );
        assert_eq!(
            db.get_setting("active_account_email").unwrap(),
            Some("user@gmail.com".to_string())
        );
    }

    // --- delete_setting ---

    #[test]
    fn should_delete_existing_setting() {
        // Given: a database with a setting
        let db = open_in_memory_db();
        db.set_setting("active_account_email", "user@gmail.com").unwrap();

        // When: deleting the setting
        let result = db.delete_setting("active_account_email");

        // Then: setting is removed
        assert!(result.is_ok());
        let value = db.get_setting("active_account_email").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn should_succeed_when_deleting_nonexistent_setting() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: deleting a setting that doesn't exist
        let result = db.delete_setting("nonexistent_key");

        // Then: succeeds without error (DELETE WHERE is idempotent)
        assert!(result.is_ok());
    }

    #[test]
    fn should_not_affect_other_settings_when_deleting() {
        // Given: a database with multiple settings
        let db = open_in_memory_db();
        db.set_setting("client_id", "id_123").unwrap();
        db.set_setting("client_secret", "secret_456").unwrap();

        // When: deleting one setting
        db.delete_setting("client_id").unwrap();

        // Then: other settings are untouched
        assert!(db.get_setting("client_id").unwrap().is_none());
        assert_eq!(
            db.get_setting("client_secret").unwrap(),
            Some("secret_456".to_string())
        );
    }

    // --- persistence across operations ---

    #[test]
    fn should_persist_data_to_file_and_reload() {
        // Given: a database file with data
        let dir = tempfile::tempdir().expect("Failed to create temp dir");
        let db_path = dir.path().join("persist_test.db");

        {
            let db = Database::open(&db_path).unwrap();
            db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
            db.set_setting("active_account_email", "user1@gmail.com").unwrap();
            db.set_setting("client_id", "my_client_id").unwrap();
        }

        // When: opening a new database connection to the same file
        let db = Database::open(&db_path).unwrap();

        // Then: previously stored data is available
        let accounts = db.load_all_accounts().unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].email, "user1@gmail.com");

        assert_eq!(
            db.get_setting("active_account_email").unwrap(),
            Some("user1@gmail.com".to_string())
        );
        assert_eq!(
            db.get_setting("client_id").unwrap(),
            Some("my_client_id".to_string())
        );
    }

    // --- edge cases ---

    #[test]
    fn should_handle_account_with_empty_tokens() {
        // Given: an account with empty token strings
        let db = open_in_memory_db();
        let account = Account {
            email: "user@gmail.com".to_string(),
            access_token: String::new(),
            refresh_token: String::new(),
            expires_at: 0,
        };

        // When: upserting and loading
        db.upsert_account(&account).unwrap();
        let accounts = db.load_all_accounts().unwrap();

        // Then: empty strings are preserved as-is
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].access_token, "");
        assert_eq!(accounts[0].refresh_token, "");
        assert_eq!(accounts[0].expires_at, 0);
    }

    #[test]
    fn should_handle_setting_with_empty_value() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: setting a key with an empty value
        db.set_setting("key", "").unwrap();

        // Then: empty value is stored and retrievable
        let value = db.get_setting("key").unwrap();
        assert_eq!(value, Some(String::new()));
    }

    #[test]
    fn should_handle_negative_expires_at() {
        // Given: an account with negative expires_at
        let db = open_in_memory_db();
        let account = Account {
            email: "user@gmail.com".to_string(),
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: -1,
        };

        // When: upserting and loading
        db.upsert_account(&account).unwrap();
        let accounts = db.load_all_accounts().unwrap();

        // Then: negative value is preserved
        assert_eq!(accounts[0].expires_at, -1);
    }

    #[test]
    fn should_handle_special_characters_in_setting_value() {
        // Given: a setting value containing special characters
        let db = open_in_memory_db();
        let value_with_special = "user+tag@gmail.com";

        // When: storing and retrieving
        db.set_setting("active_account_email", value_with_special).unwrap();

        // Then: value is preserved exactly
        assert_eq!(
            db.get_setting("active_account_email").unwrap(),
            Some(value_with_special.to_string())
        );
    }

    #[test]
    fn should_handle_email_with_special_characters() {
        // Given: an account with special characters in email
        let db = open_in_memory_db();
        let account = Account {
            email: "user+tag@gmail.com".to_string(),
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: 1700000000,
        };

        // When: upserting and loading
        db.upsert_account(&account).unwrap();
        let accounts = db.load_all_accounts().unwrap();

        // Then: email is preserved exactly
        assert_eq!(accounts[0].email, "user+tag@gmail.com");
    }

    // --- regression: operation-order (DB-first write-through pattern) ---

    #[test]
    fn should_reflect_delete_in_db_immediately_regardless_of_memory() {
        // Regression: remove_account must delete from DB before in-memory
        // to prevent account resurrection on restart if in-memory succeeds but DB fails.
        // Given: a database with an account and an active_account_email setting
        let db = open_in_memory_db();
        db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
        db.set_setting("active_account_email", "user1@gmail.com").unwrap();

        // When: DB delete operations are performed (simulating DB-first pattern)
        db.delete_account("user1@gmail.com").unwrap();
        db.delete_setting("active_account_email").unwrap();

        // Then: DB reflects the deletion immediately
        let accounts = db.load_all_accounts().unwrap();
        assert!(accounts.is_empty());
        assert!(db.get_setting("active_account_email").unwrap().is_none());
    }

    #[test]
    fn should_reflect_setting_update_in_db_immediately() {
        // Regression: switch_account must update DB before in-memory
        // to prevent active account rollback on restart if DB write fails.
        // Given: a database with two accounts and an active setting
        let db = open_in_memory_db();
        db.upsert_account(&create_test_account("user1@gmail.com")).unwrap();
        db.upsert_account(&create_test_account("user2@gmail.com")).unwrap();
        db.set_setting("active_account_email", "user1@gmail.com").unwrap();

        // When: DB setting is updated (simulating DB-first pattern)
        db.set_setting("active_account_email", "user2@gmail.com").unwrap();

        // Then: DB reflects the new active account immediately
        assert_eq!(
            db.get_setting("active_account_email").unwrap(),
            Some("user2@gmail.com".to_string())
        );
    }

    // --- regression: cross-validation (empty credential rejection) ---

    #[test]
    fn should_store_and_retrieve_non_empty_credentials() {
        // Regression: set_oauth_credentials must reject empty strings at command level.
        // This test verifies DB correctly stores valid credentials.
        let db = open_in_memory_db();

        // When: storing non-empty credentials
        db.set_setting("client_id", "valid_id_123").unwrap();
        db.set_setting("client_secret", "valid_secret_456").unwrap();

        // Then: credentials are retrievable
        assert_eq!(
            db.get_setting("client_id").unwrap(),
            Some("valid_id_123".to_string())
        );
        assert_eq!(
            db.get_setting("client_secret").unwrap(),
            Some("valid_secret_456".to_string())
        );
    }
}
