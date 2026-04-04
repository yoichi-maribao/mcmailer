use mcmailer_lib::db::Database;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    fn open_in_memory_db() -> Database {
        Database::open(Path::new(":memory:")).expect("Failed to open in-memory database")
    }

    // --- upsert_watch_state / get_watch_state ---

    #[test]
    fn should_insert_and_retrieve_watch_state() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: upserting a watch state entry
        db.upsert_watch_state("user@gmail.com", "12345", 1700006400)
            .unwrap();

        // Then: the entry can be retrieved
        let state = db.get_watch_state("user@gmail.com").unwrap();
        assert!(state.is_some());
        let (history_id, expiration) = state.unwrap();
        assert_eq!(history_id, "12345");
        assert_eq!(expiration, 1700006400);
    }

    #[test]
    fn should_return_none_for_nonexistent_watch_state() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: querying a non-existent email
        let state = db.get_watch_state("nobody@gmail.com").unwrap();

        // Then: returns None
        assert!(state.is_none());
    }

    #[test]
    fn should_update_existing_watch_state_on_upsert() {
        // Given: a database with an existing watch state
        let db = open_in_memory_db();
        db.upsert_watch_state("user@gmail.com", "100", 1700000000)
            .unwrap();

        // When: upserting the same email with updated values
        db.upsert_watch_state("user@gmail.com", "200", 1800000000)
            .unwrap();

        // Then: updated values are returned
        let state = db.get_watch_state("user@gmail.com").unwrap();
        let (history_id, expiration) = state.unwrap();
        assert_eq!(history_id, "200");
        assert_eq!(expiration, 1800000000);
    }

    #[test]
    fn should_store_multiple_watch_states_independently() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: inserting watch states for different accounts
        db.upsert_watch_state("alice@gmail.com", "100", 1700000000)
            .unwrap();
        db.upsert_watch_state("bob@gmail.com", "200", 1800000000)
            .unwrap();

        // Then: each account's state is independent
        let alice = db.get_watch_state("alice@gmail.com").unwrap().unwrap();
        assert_eq!(alice.0, "100");
        assert_eq!(alice.1, 1700000000);

        let bob = db.get_watch_state("bob@gmail.com").unwrap().unwrap();
        assert_eq!(bob.0, "200");
        assert_eq!(bob.1, 1800000000);
    }

    // --- delete_watch_state ---

    #[test]
    fn should_delete_existing_watch_state() {
        // Given: a database with a watch state
        let db = open_in_memory_db();
        db.upsert_watch_state("user@gmail.com", "100", 1700000000)
            .unwrap();

        // When: deleting the watch state
        let result = db.delete_watch_state("user@gmail.com");

        // Then: succeeds and entry is gone
        assert!(result.is_ok());
        assert!(db.get_watch_state("user@gmail.com").unwrap().is_none());
    }

    #[test]
    fn should_succeed_when_deleting_nonexistent_watch_state() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: deleting a non-existent watch state
        let result = db.delete_watch_state("nobody@gmail.com");

        // Then: succeeds without error (idempotent)
        assert!(result.is_ok());
    }

    #[test]
    fn should_not_affect_other_watch_states_when_deleting() {
        // Given: a database with two watch states
        let db = open_in_memory_db();
        db.upsert_watch_state("alice@gmail.com", "100", 1700000000)
            .unwrap();
        db.upsert_watch_state("bob@gmail.com", "200", 1800000000)
            .unwrap();

        // When: deleting one watch state
        db.delete_watch_state("alice@gmail.com").unwrap();

        // Then: the other watch state is untouched
        assert!(db.get_watch_state("alice@gmail.com").unwrap().is_none());
        let bob = db.get_watch_state("bob@gmail.com").unwrap().unwrap();
        assert_eq!(bob.0, "200");
    }

    // --- load_all_watch_states ---

    #[test]
    fn should_return_empty_vec_when_no_watch_states() {
        // Given: an empty database
        let db = open_in_memory_db();

        // When: loading all watch states
        let states = db.load_all_watch_states().unwrap();

        // Then: returns empty vec
        assert!(states.is_empty());
    }

    #[test]
    fn should_load_all_stored_watch_states() {
        // Given: a database with multiple watch states
        let db = open_in_memory_db();
        db.upsert_watch_state("alice@gmail.com", "100", 1700000000)
            .unwrap();
        db.upsert_watch_state("bob@gmail.com", "200", 1800000000)
            .unwrap();
        db.upsert_watch_state("carol@gmail.com", "300", 1900000000)
            .unwrap();

        // When: loading all watch states
        let states = db.load_all_watch_states().unwrap();

        // Then: all entries are returned
        assert_eq!(states.len(), 3);
        let emails: Vec<&str> = states.iter().map(|(e, _, _)| e.as_str()).collect();
        assert!(emails.contains(&"alice@gmail.com"));
        assert!(emails.contains(&"bob@gmail.com"));
        assert!(emails.contains(&"carol@gmail.com"));
    }

    // --- edge cases ---

    #[test]
    fn should_handle_email_with_special_characters_in_watch_state() {
        // Given: an email with special characters
        let db = open_in_memory_db();
        let email = "user+notifications@gmail.com";

        // When: storing and retrieving
        db.upsert_watch_state(email, "500", 1700000000).unwrap();
        let state = db.get_watch_state(email).unwrap();

        // Then: email is preserved exactly
        assert!(state.is_some());
        assert_eq!(state.unwrap().0, "500");
    }

    #[test]
    fn should_handle_large_history_id_string() {
        // Given: a very large history ID value
        let db = open_in_memory_db();

        // When: storing a large history ID
        db.upsert_watch_state("user@gmail.com", "99999999999999", 1700000000)
            .unwrap();
        let state = db.get_watch_state("user@gmail.com").unwrap();

        // Then: large value is preserved
        assert_eq!(state.unwrap().0, "99999999999999");
    }

    // --- persistence across operations ---

    #[test]
    fn should_persist_watch_state_to_file_and_reload() {
        // Given: a database file with watch state data
        let dir = tempfile::tempdir().expect("Failed to create temp dir");
        let db_path = dir.path().join("watch_test.db");

        {
            let db = Database::open(&db_path).unwrap();
            db.upsert_watch_state("user@gmail.com", "12345", 1700006400)
                .unwrap();
        }

        // When: opening a new connection to the same file
        let db = Database::open(&db_path).unwrap();

        // Then: previously stored watch state is available
        let state = db.get_watch_state("user@gmail.com").unwrap();
        assert!(state.is_some());
        let (history_id, expiration) = state.unwrap();
        assert_eq!(history_id, "12345");
        assert_eq!(expiration, 1700006400);
    }
}
