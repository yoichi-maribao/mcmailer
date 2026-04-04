import { describe, it, expect } from "vitest";
import {
  EVENT_NEW_MAIL_RECEIVED,
  EVENT_NAVIGATE_TO_MAIL,
} from "../events";
import type { NewMailEvent, NavigateToMailEvent } from "../events";

describe("events", () => {
  // --- Event name constants ---

  describe("event name constants", () => {
    it("should define new-mail-received event name", () => {
      // Given/When: accessing the constant
      // Then: value matches the Rust-side event name
      expect(EVENT_NEW_MAIL_RECEIVED).toBe("new-mail-received");
    });

    it("should define navigate-to-mail event name", () => {
      // Given/When: accessing the constant
      // Then: value matches the Rust-side event name
      expect(EVENT_NAVIGATE_TO_MAIL).toBe("navigate-to-mail");
    });
  });

  // --- Event payload type shapes ---

  describe("NewMailEvent type shape", () => {
    it("should accept valid new mail event payload", () => {
      // Given: a payload matching the NewMailEvent interface
      const event: NewMailEvent = {
        accountEmail: "user@gmail.com",
        messageId: "msg_123",
        subject: "New Message",
        from: "sender@example.com",
      };

      // When/Then: all required fields are accessible
      expect(event.accountEmail).toBe("user@gmail.com");
      expect(event.messageId).toBe("msg_123");
      expect(event.subject).toBe("New Message");
      expect(event.from).toBe("sender@example.com");
    });
  });

  describe("NavigateToMailEvent type shape", () => {
    it("should accept valid navigate-to-mail event payload", () => {
      // Given: a payload matching the NavigateToMailEvent interface
      const event: NavigateToMailEvent = {
        accountEmail: "user@gmail.com",
        messageId: "msg_456",
      };

      // When/Then: all required fields are accessible
      expect(event.accountEmail).toBe("user@gmail.com");
      expect(event.messageId).toBe("msg_456");
    });
  });
});
