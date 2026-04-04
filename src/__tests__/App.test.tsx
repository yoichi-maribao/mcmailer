import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor, fireEvent } from "@testing-library/react";

const mockUseAuth = vi.fn();
const mockUseAccounts = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

vi.mock("../features/auth/useAuth", () => ({
  useAuth: () => mockUseAuth(),
}));

vi.mock("../features/accounts/useAccounts", () => ({
  useAccounts: () => mockUseAccounts(),
}));

vi.mock("../features/mail/useMails", () => ({
  useMails: () => ({
    messages: [],
    hasMore: false,
    isLoading: false,
    error: null,
    loadMore: vi.fn(),
    refresh: vi.fn(),
    getMessageDetail: vi.fn(),
  }),
}));

vi.mock("../shared/commands", async (importOriginal) => {
  const actual = await importOriginal<Record<string, unknown>>();
  return {
    ...actual,
    hasOAuthCredentials: vi.fn().mockResolvedValue(true),
    setOAuthCredentials: vi.fn().mockResolvedValue(undefined),
  };
});

import App from "../App";

describe("App", () => {
  beforeEach(() => {
    mockUseAuth.mockReset();
    mockUseAccounts.mockReset();
    mockUseAccounts.mockReturnValue({
      accounts: [],
      activeAccount: null,
      isLoading: false,
      error: null,
      switchAccount: vi.fn(),
      addAccount: vi.fn(),
    });
  });

  // --- Routing based on auth state ---

  it("should show auth screen when not authenticated", async () => {
    // Given: user is not authenticated
    mockUseAuth.mockReturnValue({
      isAuthenticated: false,
      isLoading: false,
      activeEmail: null,
      login: vi.fn(),
      logout: vi.fn(),
      error: null,
    });

    // When: rendering the app
    render(<App />);

    // Then: login button is visible (auth screen is shown)
    await waitFor(() => {
      expect(
        screen.getByRole("button", {
          name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i,
        }),
      ).toBeInTheDocument();
    });
  });

  it("should show mail screen when authenticated", async () => {
    // Given: user is authenticated
    mockUseAuth.mockReturnValue({
      isAuthenticated: true,
      isLoading: false,
      activeEmail: "user@gmail.com",
      login: vi.fn(),
      logout: vi.fn(),
      error: null,
    });
    mockUseAccounts.mockReturnValue({
      accounts: [{ email: "user@gmail.com", isActive: true }],
      activeAccount: { email: "user@gmail.com", isActive: true },
      isLoading: false,
      error: null,
      switchAccount: vi.fn(),
      addAccount: vi.fn(),
    });

    // When: rendering the app
    render(<App />);

    // Then: mail-related UI is visible (not login screen)
    await waitFor(() => {
      expect(
        screen.queryByRole("button", {
          name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i,
        }),
      ).not.toBeInTheDocument();
    });
  });

  it("should show loading screen while checking auth", () => {
    // Given: auth state is loading
    mockUseAuth.mockReturnValue({
      isAuthenticated: false,
      isLoading: true,
      activeEmail: null,
      login: vi.fn(),
      logout: vi.fn(),
      error: null,
    });

    // When: rendering the app
    render(<App />);

    // Then: loading indicator is shown
    expect(screen.getByRole("progressbar")).toBeInTheDocument();
  });

  it("should display active account email when authenticated", async () => {
    // Given: user is authenticated
    mockUseAuth.mockReturnValue({
      isAuthenticated: true,
      isLoading: false,
      activeEmail: "myaccount@gmail.com",
      login: vi.fn(),
      logout: vi.fn(),
      error: null,
    });
    mockUseAccounts.mockReturnValue({
      accounts: [{ email: "myaccount@gmail.com", isActive: true }],
      activeAccount: { email: "myaccount@gmail.com", isActive: true },
      isLoading: false,
      error: null,
      switchAccount: vi.fn(),
      addAccount: vi.fn(),
    });

    // When: rendering the app
    render(<App />);

    // Then: the active email is visible via account dropdown
    fireEvent.click(screen.getByRole("button", { name: /アカウント|account/i }));
    await waitFor(() => {
      expect(screen.getByText("myaccount@gmail.com")).toBeInTheDocument();
    });
  });
});
