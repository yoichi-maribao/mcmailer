import { invoke } from "@tauri-apps/api/core";
import type { MessageListResponse, MessageDetail, AccountInfo } from "./types";

export function isTauriEnvironment(): boolean {
  return (
    typeof window !== "undefined" && "__TAURI_INTERNALS__" in window
  );
}

function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriEnvironment()) {
    return Promise.reject(
      new Error("この機能はデスクトップアプリでのみ利用できます"),
    );
  }
  return args !== undefined ? invoke<T>(cmd, args) : invoke<T>(cmd);
}

export function startOAuth(): Promise<void> {
  return safeInvoke("start_oauth");
}

export function listMessages(
  pageToken: string | null,
): Promise<MessageListResponse> {
  return safeInvoke("list_messages", { pageToken });
}

export function getMessage(id: string): Promise<MessageDetail> {
  return safeInvoke("get_message", { id });
}

export function listAccounts(): Promise<AccountInfo[]> {
  return safeInvoke("list_accounts");
}

export function switchAccount(email: string): Promise<void> {
  return safeInvoke("switch_account", { email });
}

export function removeAccount(email: string): Promise<void> {
  return safeInvoke("remove_account", { email });
}

export function getActiveAccount(): Promise<AccountInfo | null> {
  return safeInvoke("get_active_account");
}

export function hasOAuthCredentials(): Promise<boolean> {
  return safeInvoke("has_oauth_credentials");
}

export function setOAuthCredentials(
  clientId: string,
  clientSecret: string,
): Promise<void> {
  return safeInvoke("set_oauth_credentials", { clientId, clientSecret });
}
