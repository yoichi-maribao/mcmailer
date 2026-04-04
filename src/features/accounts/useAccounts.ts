import { useState, useEffect, useCallback } from "react";
import {
  listAccounts,
  switchAccount,
  startOAuth,
} from "../../shared/commands";
import type { AccountInfo } from "../../shared/types";

interface AccountsState {
  accounts: AccountInfo[];
  activeAccount: AccountInfo | null;
  isLoading: boolean;
  error: string | null;
  switchAccount: (email: string) => Promise<void>;
  addAccount: () => Promise<void>;
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

  return {
    accounts,
    activeAccount,
    isLoading,
    error,
    switchAccount: switchAccountFn,
    addAccount,
  };
}
