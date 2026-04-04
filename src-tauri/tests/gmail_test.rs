use mcmailer_lib::gmail::{
    parse_message_headers, extract_body_from_payload,
    GmailMessage, GmailMessageListResponse, MessageHeader, MessagePayload, MessagePart,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message_headers() -> Vec<MessageHeader> {
        vec![
            MessageHeader {
                name: "From".to_string(),
                value: "sender@gmail.com".to_string(),
            },
            MessageHeader {
                name: "Subject".to_string(),
                value: "Test Email Subject".to_string(),
            },
            MessageHeader {
                name: "Date".to_string(),
                value: "Mon, 1 Jan 2026 12:00:00 +0000".to_string(),
            },
        ]
    }

    // --- parse_message_headers ---

    #[test]
    fn should_extract_from_header() {
        // Given: message headers containing a From field
        let headers = create_test_message_headers();

        // When: parsing the From header
        let from = parse_message_headers(&headers, "From");

        // Then: returns the sender email
        assert_eq!(from, Some("sender@gmail.com".to_string()));
    }

    #[test]
    fn should_extract_subject_header() {
        // Given: message headers containing a Subject field
        let headers = create_test_message_headers();

        // When: parsing the Subject header
        let subject = parse_message_headers(&headers, "Subject");

        // Then: returns the subject text
        assert_eq!(subject, Some("Test Email Subject".to_string()));
    }

    #[test]
    fn should_return_none_for_missing_header() {
        // Given: message headers without a To field
        let headers = create_test_message_headers();

        // When: parsing a non-existent header
        let to = parse_message_headers(&headers, "To");

        // Then: returns None
        assert_eq!(to, None);
    }

    #[test]
    fn should_be_case_sensitive_for_header_names() {
        // Given: message headers with standard casing
        let headers = create_test_message_headers();

        // When: querying with wrong case
        let result = parse_message_headers(&headers, "from");

        // Then: returns None (header names are case-sensitive per spec matching)
        assert_eq!(result, None);
    }

    // --- extract_body_from_payload ---

    #[test]
    fn should_extract_plain_text_body_from_single_part() {
        // Given: a payload with plain text body
        let payload = MessagePayload {
            mime_type: "text/plain".to_string(),
            body: Some(MessagePart {
                data: Some(base64_url_encode("Hello, this is a test email.")),
                size: 28,
            }),
            parts: None,
            headers: vec![],
        };

        // When: extracting the body
        let (body, content_type) = extract_body_from_payload(&payload);

        // Then: returns the decoded plain text body
        assert_eq!(body, "Hello, this is a test email.");
        assert_eq!(content_type, "text/plain");
    }

    #[test]
    fn should_extract_html_body_from_single_part() {
        // Given: a payload with HTML body
        let html_content = "<html><body><p>Hello</p></body></html>";
        let payload = MessagePayload {
            mime_type: "text/html".to_string(),
            body: Some(MessagePart {
                data: Some(base64_url_encode(html_content)),
                size: html_content.len() as u64,
            }),
            parts: None,
            headers: vec![],
        };

        // When: extracting the body
        let (body, content_type) = extract_body_from_payload(&payload);

        // Then: returns the decoded HTML body
        assert_eq!(body, html_content);
        assert_eq!(content_type, "text/html");
    }

    #[test]
    fn should_prefer_html_body_from_multipart_alternative() {
        // Given: a multipart/alternative payload with both text and HTML
        let plain_text = "Plain text version";
        let html_content = "<html><body><p>HTML version</p></body></html>";
        let payload = MessagePayload {
            mime_type: "multipart/alternative".to_string(),
            body: None,
            parts: Some(vec![
                MessagePayload {
                    mime_type: "text/plain".to_string(),
                    body: Some(MessagePart {
                        data: Some(base64_url_encode(plain_text)),
                        size: plain_text.len() as u64,
                    }),
                    parts: None,
                    headers: vec![],
                },
                MessagePayload {
                    mime_type: "text/html".to_string(),
                    body: Some(MessagePart {
                        data: Some(base64_url_encode(html_content)),
                        size: html_content.len() as u64,
                    }),
                    parts: None,
                    headers: vec![],
                },
            ]),
            headers: vec![],
        };

        // When: extracting the body
        let (body, content_type) = extract_body_from_payload(&payload);

        // Then: prefers the HTML version
        assert_eq!(body, html_content);
        assert_eq!(content_type, "text/html");
    }

    #[test]
    fn should_fallback_to_plain_text_when_no_html_in_multipart() {
        // Given: a multipart payload with only plain text
        let plain_text = "Only plain text";
        let payload = MessagePayload {
            mime_type: "multipart/alternative".to_string(),
            body: None,
            parts: Some(vec![MessagePayload {
                mime_type: "text/plain".to_string(),
                body: Some(MessagePart {
                    data: Some(base64_url_encode(plain_text)),
                    size: plain_text.len() as u64,
                }),
                parts: None,
                headers: vec![],
            }]),
            headers: vec![],
        };

        // When: extracting the body
        let (body, content_type) = extract_body_from_payload(&payload);

        // Then: returns the plain text
        assert_eq!(body, plain_text);
        assert_eq!(content_type, "text/plain");
    }

    #[test]
    fn should_return_empty_body_when_payload_has_no_data() {
        // Given: a payload with no body data
        let payload = MessagePayload {
            mime_type: "text/plain".to_string(),
            body: Some(MessagePart {
                data: None,
                size: 0,
            }),
            parts: None,
            headers: vec![],
        };

        // When: extracting the body
        let (body, _) = extract_body_from_payload(&payload);

        // Then: returns empty string
        assert_eq!(body, "");
    }

    // --- GmailMessageListResponse deserialization ---

    #[test]
    fn should_deserialize_message_list_response() {
        // Given: a Gmail API messages.list response JSON
        let json = r#"{
            "messages": [
                {"id": "msg1", "threadId": "thread1"},
                {"id": "msg2", "threadId": "thread2"}
            ],
            "nextPageToken": "token123",
            "resultSizeEstimate": 2
        }"#;

        // When: deserializing
        let response: GmailMessageListResponse = serde_json::from_str(json).unwrap();

        // Then: messages are correctly parsed
        assert_eq!(response.messages.len(), 2);
        assert_eq!(response.messages[0].id, "msg1");
        assert_eq!(response.messages[1].id, "msg2");
        assert_eq!(response.next_page_token, Some("token123".to_string()));
    }

    #[test]
    fn should_deserialize_empty_message_list_response() {
        // Given: a Gmail API response with no messages
        let json = r#"{
            "messages": [],
            "resultSizeEstimate": 0
        }"#;

        // When: deserializing
        let response: GmailMessageListResponse = serde_json::from_str(json).unwrap();

        // Then: empty messages list
        assert!(response.messages.is_empty());
        assert_eq!(response.next_page_token, None);
    }

    #[test]
    fn should_deserialize_full_message_response() {
        // Given: a Gmail API messages.get response JSON
        let json = r#"{
            "id": "msg123",
            "threadId": "thread456",
            "labelIds": ["INBOX", "UNREAD"],
            "snippet": "Preview text here...",
            "payload": {
                "mimeType": "text/plain",
                "headers": [
                    {"name": "From", "value": "sender@example.com"},
                    {"name": "Subject", "value": "Test Subject"}
                ],
                "body": {
                    "data": "SGVsbG8gV29ybGQ",
                    "size": 11
                }
            },
            "internalDate": "1700000000000"
        }"#;

        // When: deserializing
        let message: GmailMessage = serde_json::from_str(json).unwrap();

        // Then: all fields are correctly parsed
        assert_eq!(message.id, "msg123");
        assert_eq!(message.thread_id, "thread456");
        assert_eq!(message.snippet, "Preview text here...");
        assert!(message.label_ids.contains(&"UNREAD".to_string()));
    }

    // --- Helper ---

    fn base64_url_encode(input: &str) -> String {
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        URL_SAFE_NO_PAD.encode(input.as_bytes())
    }
}
