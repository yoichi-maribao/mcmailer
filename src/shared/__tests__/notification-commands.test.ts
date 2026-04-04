import { describe, it, expect, vi, beforeEach, beforeAll, afterAll } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
import {
  getNotificationSettings,
  setNotificationSettings,
} from "../commands";

const mockInvoke = vi.mocked(invoke);

describe("notification commands", () => {
  beforeAll(() => {
    (window as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  });

  afterAll(() => {
    delete (window as Record<string, unknown>).__TAURI_INTERNALS__;
  });

  beforeEach(() => {
    mockInvoke.mockReset();
  });

  // --- getNotificationSettings ---

  describe("getNotificationSettings", () => {
    it("should invoke get_notification_settings command", async () => {
      // Given: backend returns notification settings
      const mockSettings = {
        enabled: true,
        pubsubSubscription: "projects/my-project/subscriptions/gmail-sub",
        pubsubTopic: "projects/my-project/topics/gmail",
      };
      mockInvoke.mockResolvedValueOnce(mockSettings);

      // When: calling getNotificationSettings
      const result = await getNotificationSettings();

      // Then: invokes the correct Tauri command and returns settings
      expect(mockInvoke).toHaveBeenCalledWith("get_notification_settings");
      expect(result).toEqual(mockSettings);
    });

    it("should propagate error when settings retrieval fails", async () => {
      // Given: invoke rejects
      mockInvoke.mockRejectedValueOnce(new Error("Database error"));

      // When/Then: getNotificationSettings throws
      await expect(getNotificationSettings()).rejects.toThrow("Database error");
    });
  });

  // --- setNotificationSettings ---

  describe("setNotificationSettings", () => {
    it("should invoke set_notification_settings with all fields", async () => {
      // Given: invoke resolves successfully
      mockInvoke.mockResolvedValueOnce(undefined);

      // When: calling setNotificationSettings
      await setNotificationSettings({
        enabled: true,
        pubsubSubscription: "projects/my-project/subscriptions/gmail-push-sub",
        pubsubTopic: "projects/my-project/topics/gmail-push",
      });

      // Then: invokes with correct arguments
      expect(mockInvoke).toHaveBeenCalledWith("set_notification_settings", {
        settings: {
          enabled: true,
          pubsubSubscription: "projects/my-project/subscriptions/gmail-push-sub",
          pubsubTopic: "projects/my-project/topics/gmail-push",
        },
      });
    });

    it("should invoke set_notification_settings with notifications disabled", async () => {
      // Given: invoke resolves successfully
      mockInvoke.mockResolvedValueOnce(undefined);

      // When: disabling notifications
      await setNotificationSettings({
        enabled: false,
        pubsubSubscription: "projects/my-project/subscriptions/gmail-sub",
        pubsubTopic: "projects/my-project/topics/gmail",
      });

      // Then: invokes with enabled=false
      expect(mockInvoke).toHaveBeenCalledWith("set_notification_settings", {
        settings: {
          enabled: false,
          pubsubSubscription: "projects/my-project/subscriptions/gmail-sub",
          pubsubTopic: "projects/my-project/topics/gmail",
        },
      });
    });

    it("should propagate error when settings update fails", async () => {
      // Given: invoke rejects
      mockInvoke.mockRejectedValueOnce(new Error("Invalid settings"));

      // When/Then: setNotificationSettings throws
      await expect(
        setNotificationSettings({
          enabled: true,
          pubsubSubscription: "",
          pubsubTopic: "",
        }),
      ).rejects.toThrow("Invalid settings");
    });
  });
});
