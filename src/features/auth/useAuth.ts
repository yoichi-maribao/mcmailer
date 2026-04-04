import { useState, useEffect, useCallback } from "react";
import {
  getActiveAccount,
  startOAuth,
  removeAccount,
} from "../../shared/commands";

interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  activeEmail: string | null;
  error: string | null;
  login: () => Promise<void>;
  logout: () => Promise<void>;
}

export function useAuth(): AuthState {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [activeEmail, setActiveEmail] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getActiveAccount()
      .then((account) => {
        if (account) {
          setIsAuthenticated(true);
          setActiveEmail(account.email);
        } else {
          setIsAuthenticated(false);
          setActiveEmail(null);
        }
      })
      .catch(() => {
        setIsAuthenticated(false);
        setActiveEmail(null);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  const login = useCallback(async () => {
    try {
      setError(null);
      await startOAuth();
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      setError(message);
      throw e;
    }
  }, []);

  const logout = useCallback(async () => {
    if (!activeEmail) return;
    await removeAccount(activeEmail);
    setIsAuthenticated(false);
    setActiveEmail(null);
  }, [activeEmail]);

  return { isAuthenticated, isLoading, activeEmail, error, login, logout };
}
