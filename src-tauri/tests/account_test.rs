use mcmailer_lib::account::{
    Account, AccountStore, add_account, list_accounts, switch_account, remove_account,
    get_active_account,
};

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

    fn create_empty_store() -> AccountStore {
        AccountStore {
            accounts: vec![],
            active_account_email: None,
        }
    }

    // --- add_account ---

    #[test]
    fn should_add_account_to_empty_store() {
        // Given: an empty account store
        let mut store = create_empty_store();
        let account = create_test_account("user1@gmail.com");

        // When: adding an account
        add_account(&mut store, account);

        // Then: store contains the account and it becomes active
        assert_eq!(store.accounts.len(), 1);
        assert_eq!(store.accounts[0].email, "user1@gmail.com");
        assert_eq!(
            store.active_account_email,
            Some("user1@gmail.com".to_string())
        );
    }

    #[test]
    fn should_add_second_account_without_changing_active() {
        // Given: a store with one active account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));

        // When: adding a second account
        add_account(&mut store, create_test_account("user2@gmail.com"));

        // Then: two accounts exist, first remains active
        assert_eq!(store.accounts.len(), 2);
        assert_eq!(
            store.active_account_email,
            Some("user1@gmail.com".to_string())
        );
    }

    #[test]
    fn should_update_existing_account_tokens_on_re_add() {
        // Given: a store with an existing account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));

        // When: adding the same account with new tokens
        let updated = Account {
            email: "user1@gmail.com".to_string(),
            access_token: "new_access_token".to_string(),
            refresh_token: "new_refresh_token".to_string(),
            expires_at: 1800000000,
        };
        add_account(&mut store, updated);

        // Then: only one account exists with updated tokens
        assert_eq!(store.accounts.len(), 1);
        assert_eq!(store.accounts[0].access_token, "new_access_token");
        assert_eq!(store.accounts[0].refresh_token, "new_refresh_token");
        assert_eq!(store.accounts[0].expires_at, 1800000000);
    }

    // --- list_accounts ---

    #[test]
    fn should_return_empty_list_when_no_accounts() {
        // Given: an empty store
        let store = create_empty_store();

        // When: listing accounts
        let accounts = list_accounts(&store);

        // Then: returns empty list
        assert!(accounts.is_empty());
    }

    #[test]
    fn should_return_all_accounts() {
        // Given: a store with multiple accounts
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));
        add_account(&mut store, create_test_account("user2@gmail.com"));

        // When: listing accounts
        let accounts = list_accounts(&store);

        // Then: returns all accounts
        assert_eq!(accounts.len(), 2);
    }

    // --- switch_account ---

    #[test]
    fn should_switch_active_account() {
        // Given: a store with two accounts, first is active
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));
        add_account(&mut store, create_test_account("user2@gmail.com"));

        // When: switching to the second account
        let result = switch_account(&mut store, "user2@gmail.com");

        // Then: second account becomes active
        assert!(result.is_ok());
        assert_eq!(
            store.active_account_email,
            Some("user2@gmail.com".to_string())
        );
    }

    #[test]
    fn should_return_error_when_switching_to_nonexistent_account() {
        // Given: a store with one account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));

        // When: switching to a non-existent account
        let result = switch_account(&mut store, "nonexistent@gmail.com");

        // Then: returns an error
        assert!(result.is_err());
    }

    // --- get_active_account ---

    #[test]
    fn should_return_active_account() {
        // Given: a store with an active account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("active@gmail.com"));

        // When: getting the active account
        let active = get_active_account(&store);

        // Then: returns the active account
        assert!(active.is_some());
        assert_eq!(active.unwrap().email, "active@gmail.com");
    }

    #[test]
    fn should_return_none_when_no_active_account() {
        // Given: an empty store
        let store = create_empty_store();

        // When: getting the active account
        let active = get_active_account(&store);

        // Then: returns None
        assert!(active.is_none());
    }

    // --- remove_account ---

    #[test]
    fn should_remove_existing_account() {
        // Given: a store with two accounts
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));
        add_account(&mut store, create_test_account("user2@gmail.com"));

        // When: removing the first account
        let result = remove_account(&mut store, "user1@gmail.com");

        // Then: only one account remains
        assert!(result.is_ok());
        assert_eq!(store.accounts.len(), 1);
        assert_eq!(store.accounts[0].email, "user2@gmail.com");
    }

    #[test]
    fn should_clear_active_when_active_account_is_removed() {
        // Given: a store with one active account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));
        assert_eq!(
            store.active_account_email,
            Some("user1@gmail.com".to_string())
        );

        // When: removing the active account
        remove_account(&mut store, "user1@gmail.com").unwrap();

        // Then: active account is cleared
        assert!(store.active_account_email.is_none());
    }

    #[test]
    fn should_return_error_when_removing_nonexistent_account() {
        // Given: a store with one account
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));

        // When: removing a non-existent account
        let result = remove_account(&mut store, "nonexistent@gmail.com");

        // Then: returns an error
        assert!(result.is_err());
    }

    // --- AccountStore serialization ---

    #[test]
    fn should_serialize_and_deserialize_account_store() {
        // Given: an account store with accounts
        let mut store = create_empty_store();
        add_account(&mut store, create_test_account("user1@gmail.com"));
        add_account(&mut store, create_test_account("user2@gmail.com"));

        // When: serializing and deserializing
        let json = serde_json::to_string(&store).unwrap();
        let deserialized: AccountStore = serde_json::from_str(&json).unwrap();

        // Then: deserialized store matches original
        assert_eq!(deserialized.accounts.len(), 2);
        assert_eq!(deserialized.active_account_email, store.active_account_email);
        assert_eq!(deserialized.accounts[0].email, "user1@gmail.com");
        assert_eq!(deserialized.accounts[1].email, "user2@gmail.com");
    }
}
