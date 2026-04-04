import { describe, it, expect, vi, beforeEach, beforeAll, afterAll } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
import {
  startOAuth,
  listMessages,
  getMessage,
  listAccounts,
  switchAccount,
  removeAccount,
  getActiveAccount,
} from "../commands";

const mockInvoke = vi.mocked(invoke);

describe("commands", () => {
  beforeAll(() => {
    // Simulate Tauri environment for tests
    (window as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  });

  afterAll(() => {
    delete (window as Record<string, unknown>).__TAURI_INTERNALS__;
  });

  beforeEach(() => {
    mockInvoke.mockReset();
  });

  // --- startOAuth ---

  describe("startOAuth", () => {
    it("should invoke start_oauth command", async () => {
      // Given: invoke resolves successfully
      mockInvoke.mockResolvedValueOnce(undefined);

      // When: calling startOAuth
      await startOAuth();

      // Then: invokes the correct Tauri command
      expect(mockInvoke).toHaveBeenCalledWith("start_oauth");
    });

    it("should propagate error when oauth fails", async () => {
      // Given: invoke rejects
      mockInvoke.mockRejectedValueOnce(new Error("OAuth failed"));

      // When/Then: startOAuth throws
      await expect(startOAuth()).rejects.toThrow("OAuth failed");
    });
  });

  // --- listMessages ---

  describe("listMessages", () => {
    it("should invoke list_messages with page token", async () => {
      // Given: API returns a message list
      const mockResponse = {
        messages: [{ id: "msg1", snippet: "Hello" }],
        nextPageToken: "token2",
      };
      mockInvoke.mockResolvedValueOnce(mockResponse);

      // When: calling listMessages with a page token
      const result = await listMessages("nextToken123");

      // Then: invokes with correct arguments and returns response
      expect(mockInvoke).toHaveBeenCalledWith("list_messages", {
        pageToken: "nextToken123",
      });
      expect(result).toEqual(mockResponse);
    });

    it("should invoke list_messages without page token for first page", async () => {
      // Given: API returns a message list
      const mockResponse = {
        messages: [{ id: "msg1", snippet: "Hello" }],
        nextPageToken: null,
      };
      mockInvoke.mockResolvedValueOnce(mockResponse);

      // When: calling listMessages without page token
      const result = await listMessages(null);

      // Then: invokes with null page token
      expect(mockInvoke).toHaveBeenCalledWith("list_messages", {
        pageToken: null,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  // --- getMessage ---

  describe("getMessage", () => {
    it("should invoke get_message with message id", async () => {
      // Given: API returns a message detail
      const mockMessage = {
        id: "msg123",
        subject: "Test",
        from: "sender@gmail.com",
        body: "<p>Hello</p>",
        contentType: "text/html",
      };
      mockInvoke.mockResolvedValueOnce(mockMessage);

      // When: calling getMessage
      const result = await getMessage("msg123");

      // Then: invokes with correct message ID
      expect(mockInvoke).toHaveBeenCalledWith("get_message", { id: "msg123" });
      expect(result).toEqual(mockMessage);
    });

    it("should propagate error when message not found", async () => {
      // Given: invoke rejects with not found error
      mockInvoke.mockRejectedValueOnce(new Error("Message not found"));

      // When/Then: getMessage throws
      await expect(getMessage("nonexistent")).rejects.toThrow(
        "Message not found",
      );
    });
  });

  // --- listAccounts ---

  describe("listAccounts", () => {
    it("should invoke list_accounts and return account list", async () => {
      // Given: API returns accounts
      const mockAccounts = [
        { email: "user1@gmail.com", isActive: true },
        { email: "user2@gmail.com", isActive: false },
      ];
      mockInvoke.mockResolvedValueOnce(mockAccounts);

      // When: calling listAccounts
      const result = await listAccounts();

      // Then: returns account list
      expect(mockInvoke).toHaveBeenCalledWith("list_accounts");
      expect(result).toEqual(mockAccounts);
    });
  });

  // --- switchAccount ---

  describe("switchAccount", () => {
    it("should invoke switch_account with email", async () => {
      // Given: switch resolves successfully
      mockInvoke.mockResolvedValueOnce(undefined);

      // When: switching account
      await switchAccount("user2@gmail.com");

      // Then: invokes with correct email
      expect(mockInvoke).toHaveBeenCalledWith("switch_account", {
        email: "user2@gmail.com",
      });
    });

    it("should propagate error when account not found", async () => {
      // Given: invoke rejects
      mockInvoke.mockRejectedValueOnce(new Error("Account not found"));

      // When/Then: switchAccount throws
      await expect(switchAccount("nonexistent@gmail.com")).rejects.toThrow(
        "Account not found",
      );
    });
  });

  // --- removeAccount ---

  describe("removeAccount", () => {
    it("should invoke remove_account with email", async () => {
      // Given: remove resolves successfully
      mockInvoke.mockResolvedValueOnce(undefined);

      // When: removing account
      await removeAccount("user1@gmail.com");

      // Then: invokes with correct email
      expect(mockInvoke).toHaveBeenCalledWith("remove_account", {
        email: "user1@gmail.com",
      });
    });
  });

  // --- getActiveAccount ---

  describe("getActiveAccount", () => {
    it("should invoke get_active_account and return account", async () => {
      // Given: API returns active account
      const mockAccount = { email: "user1@gmail.com", isActive: true };
      mockInvoke.mockResolvedValueOnce(mockAccount);

      // When: calling getActiveAccount
      const result = await getActiveAccount();

      // Then: returns the active account
      expect(mockInvoke).toHaveBeenCalledWith("get_active_account");
      expect(result).toEqual(mockAccount);
    });

    it("should return null when no active account", async () => {
      // Given: API returns null
      mockInvoke.mockResolvedValueOnce(null);

      // When: calling getActiveAccount
      const result = await getActiveAccount();

      // Then: returns null
      expect(mockInvoke).toHaveBeenCalledWith("get_active_account");
      expect(result).toBeNull();
    });
  });
});
