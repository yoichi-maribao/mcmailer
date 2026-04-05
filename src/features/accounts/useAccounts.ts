import { useState, useEffect, useCallback } from "react";
import {
  listAccounts,
  switchAccount,
  startOAuth,
  removeAccount,
} from "../../shared/commands";
import type { AccountInfo } from "../../shared/types";

interface RemoveResult {
  remaining: AccountInfo[];
}

interface AccountsState {
  accounts: AccountInfo[];
  activeAccount: AccountInfo | null;
  isLoading: boolean;
  error: string | null;
  switchAccount: (email: string) => Promise<void>;
  addAccount: () => Promise<void>;
  removeCurrentAccount: () => Promise<RemoveResult>;
}

export function useAccounts(): AccountsState {
  const [accounts, setAccounts] = useState<AccountInfo[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadAccounts = useCallback(async () => {
    const result = await listAccounts();
    if (!Array.isArray(result)) {
      throw new Error("Invalid response from list_accounts");
    }
    setAccounts(result);
  }, []);

  useEffect(() => {
    loadAccounts()
      .catch((e) => {
        setError(e instanceof Error ? e.message : String(e));
      })
      .finally(() => setIsLoading(false));
  }, [loadAccounts]);

  const activeAccount = accounts.find((a) => a.isActive) ?? null;

  const switchAccountFn = useCallback(
    async (email: string) => {
      try {
        setError(null);
        await switchAccount(email);
        await loadAccounts();
      } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        setError(message);
        throw e;
      }
    },
    [loadAccounts],
  );

  const addAccount = useCallback(async () => {
    await startOAuth();
  }, []);

  const removeCurrentAccount = useCallback(async (): Promise<RemoveResult> => {
    const current = accounts.find((a) => a.isActive);
    if (!current) {
      throw new Error("No active account to remove");
    }
    try {
      setError(null);
      await removeAccount(current.email);
      const updated = await listAccounts();
      if (!Array.isArray(updated)) {
        throw new Error("Invalid response from list_accounts");
      }
      if (updated.length > 0) {
        await switchAccount(updated[0].email);
        const refreshed = await listAccounts();
        if (!Array.isArray(refreshed)) {
          throw new Error("Invalid response from list_accounts");
        }
        setAccounts(refreshed);
        return { remaining: refreshed };
      }
      setAccounts([]);
      return { remaining: [] };
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      setError(message);
      throw e;
    }
  }, [accounts]);

  return {
    accounts,
    activeAccount,
    isLoading,
    error,
    switchAccount: switchAccountFn,
    addAccount,
    removeCurrentAccount,
  };
}
