use mcmailer_lib::history::{HistoryListResponse, HistoryEntry, HistoryMessage};

#[cfg(test)]
mod tests {
    use super::*;

    // --- HistoryListResponse deserialization ---

    #[test]
    fn should_deserialize_history_list_response_with_new_messages() {
        // Given: a Gmail history.list API response with messagesAdded entries
        let json = r#"{
            "history": [
                {
                    "id": "12345",
                    "messagesAdded": [
                        {
                            "message": {
                                "id": "msg_abc",
                                "threadId": "thread_1",
                                "labelIds": ["INBOX", "UNREAD"]
                            }
                        }
                    ]
                }
            ],
            "historyId": "12350"
        }"#;

        // When: deserializing
        let response: HistoryListResponse = serde_json::from_str(json).unwrap();

        // Then: history entries and next historyId are parsed
        assert!(response.history.is_some());
        let history = response.history.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, "12345");
        assert_eq!(response.history_id, "12350");
    }

    #[test]
    fn should_deserialize_history_list_response_with_no_changes() {
        // Given: a Gmail history.list response with no history entries
        let json = r#"{
            "historyId": "99999"
        }"#;

        // When: deserializing
        let response: HistoryListResponse = serde_json::from_str(json).unwrap();

        // Then: history is None, historyId is updated
        assert!(response.history.is_none());
        assert_eq!(response.history_id, "99999");
    }

    #[test]
    fn should_deserialize_history_list_response_with_empty_history_array() {
        // Given: a response with an empty history array
        let json = r#"{
            "history": [],
            "historyId": "55555"
        }"#;

        // When: deserializing
        let response: HistoryListResponse = serde_json::from_str(json).unwrap();

        // Then: history is Some with empty vec
        assert!(response.history.is_some());
        assert!(response.history.unwrap().is_empty());
        assert_eq!(response.history_id, "55555");
    }

    #[test]
    fn should_deserialize_multiple_history_entries() {
        // Given: a response with multiple history entries each containing new messages
        let json = r#"{
            "history": [
                {
                    "id": "100",
                    "messagesAdded": [
                        {
                            "message": {
                                "id": "msg_1",
                                "threadId": "thread_1",
                                "labelIds": ["INBOX"]
                            }
                        }
                    ]
                },
                {
                    "id": "101",
                    "messagesAdded": [
                        {
                            "message": {
                                "id": "msg_2",
                                "threadId": "thread_2",
                                "labelIds": ["INBOX", "UNREAD"]
                            }
                        },
                        {
                            "message": {
                                "id": "msg_3",
                                "threadId": "thread_3",
                                "labelIds": ["INBOX"]
                            }
                        }
                    ]
                }
            ],
            "historyId": "102"
        }"#;

        // When: deserializing
        let response: HistoryListResponse = serde_json::from_str(json).unwrap();

        // Then: all entries and their messages are parsed
        let history = response.history.unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].messages_added.as_ref().unwrap().len(), 1);
        assert_eq!(history[1].messages_added.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn should_extract_message_ids_from_history_entry() {
        // Given: a history entry with messagesAdded
        let json = r#"{
            "id": "200",
            "messagesAdded": [
                {
                    "message": {
                        "id": "msg_new_1",
                        "threadId": "thread_a",
                        "labelIds": ["INBOX", "UNREAD"]
                    }
                },
                {
                    "message": {
                        "id": "msg_new_2",
                        "threadId": "thread_b",
                        "labelIds": ["INBOX"]
                    }
                }
            ]
        }"#;

        // When: deserializing
        let entry: HistoryEntry = serde_json::from_str(json).unwrap();

        // Then: message IDs are accessible
        let messages = entry.messages_added.unwrap();
        assert_eq!(messages[0].message.id, "msg_new_1");
        assert_eq!(messages[1].message.id, "msg_new_2");
    }

    #[test]
    fn should_parse_label_ids_from_history_message() {
        // Given: a history message with multiple labels
        let json = r#"{
            "id": "msg_labeled",
            "threadId": "thread_x",
            "labelIds": ["INBOX", "UNREAD", "CATEGORY_PERSONAL"]
        }"#;

        // When: deserializing
        let message: HistoryMessage = serde_json::from_str(json).unwrap();

        // Then: all labels are present
        assert_eq!(message.label_ids.len(), 3);
        assert!(message.label_ids.contains(&"INBOX".to_string()));
        assert!(message.label_ids.contains(&"UNREAD".to_string()));
        assert!(message.label_ids.contains(&"CATEGORY_PERSONAL".to_string()));
    }

    #[test]
    fn should_handle_history_entry_without_messages_added() {
        // Given: a history entry with no messagesAdded (e.g. label change only)
        let json = r#"{
            "id": "300"
        }"#;

        // When: deserializing
        let entry: HistoryEntry = serde_json::from_str(json).unwrap();

        // Then: messagesAdded is None
        assert!(entry.messages_added.is_none());
    }

    #[test]
    fn should_handle_history_message_with_empty_label_ids() {
        // Given: a history message with no labels
        let json = r#"{
            "id": "msg_no_labels",
            "threadId": "thread_y",
            "labelIds": []
        }"#;

        // When: deserializing
        let message: HistoryMessage = serde_json::from_str(json).unwrap();

        // Then: label_ids is empty
        assert!(message.label_ids.is_empty());
    }

    #[test]
    fn should_handle_history_message_without_label_ids_field() {
        // Given: a history message where labelIds is absent
        let json = r#"{
            "id": "msg_missing_labels",
            "threadId": "thread_z"
        }"#;

        // When: deserializing
        let message: HistoryMessage = serde_json::from_str(json).unwrap();

        // Then: label_ids defaults to empty vec
        assert!(message.label_ids.is_empty());
    }
}
