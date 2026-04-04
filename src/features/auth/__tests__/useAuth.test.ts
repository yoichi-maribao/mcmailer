import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";

const mockGetActiveAccount = vi.fn();
const mockStartOAuth = vi.fn();
const mockRemoveAccount = vi.fn();

vi.mock("../../../shared/commands", () => ({
  getActiveAccount: (...args: unknown[]) => mockGetActiveAccount(...args),
  startOAuth: (...args: unknown[]) => mockStartOAuth(...args),
  removeAccount: (...args: unknown[]) => mockRemoveAccount(...args),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

import { useAuth } from "../useAuth";

describe("useAuth", () => {
  beforeEach(() => {
    mockGetActiveAccount.mockReset();
    mockStartOAuth.mockReset();
    mockRemoveAccount.mockReset();
  });

  // --- Initial state ---

  it("should start with loading state while checking auth", () => {
    // Given: getActiveAccount is pending (hasn't resolved yet)
    mockGetActiveAccount.mockReturnValue(new Promise(() => {}));

    // When: rendering the hook
    const { result } = renderHook(() => useAuth());

    // Then: loading is true, not authenticated
    expect(result.current.isLoading).toBe(true);
    expect(result.current.isAuthenticated).toBe(false);
  });

  it("should be authenticated when active account exists", async () => {
    // Given: an active account exists in the backend
    mockGetActiveAccount.mockResolvedValueOnce({
      email: "user@gmail.com",
      isActive: true,
    });

    // When: rendering the hook and waiting for async resolution
    const { result } = renderHook(() => useAuth());
    await act(async () => {});

    // Then: authenticated with the user's email
    expect(result.current.isAuthenticated).toBe(true);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.activeEmail).toBe("user@gmail.com");
  });

  it("should not be authenticated when no active account", async () => {
    // Given: no active account in the backend
    mockGetActiveAccount.mockResolvedValueOnce(null);

    // When: rendering the hook and waiting for async resolution
    const { result } = renderHook(() => useAuth());
    await act(async () => {});

    // Then: not authenticated
    expect(result.current.isAuthenticated).toBe(false);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.activeEmail).toBeNull();
  });

  // --- login ---

  it("should call startOAuth on login", async () => {
    // Given: hook is initialized as unauthenticated
    mockGetActiveAccount.mockResolvedValueOnce(null);
    const { result } = renderHook(() => useAuth());
    await act(async () => {});

    // When: calling login
    mockStartOAuth.mockResolvedValueOnce(undefined);
    await act(async () => {
      await result.current.login();
    });

    // Then: startOAuth was called
    expect(mockStartOAuth).toHaveBeenCalled();
  });

  // --- logout ---

  it("should call removeAccount and clear auth state on logout", async () => {
    // Given: authenticated user
    mockGetActiveAccount.mockResolvedValueOnce({
      email: "user@gmail.com",
      isActive: true,
    });
    const { result } = renderHook(() => useAuth());
    await act(async () => {});
    expect(result.current.isAuthenticated).toBe(true);

    // When: calling logout
    mockRemoveAccount.mockResolvedValueOnce(undefined);
    await act(async () => {
      await result.current.logout();
    });

    // Then: removeAccount was called and state is cleared
    expect(mockRemoveAccount).toHaveBeenCalledWith("user@gmail.com");
    expect(result.current.isAuthenticated).toBe(false);
    expect(result.current.activeEmail).toBeNull();
  });

  // --- Error handling ---

  it("should set error state when login fails", async () => {
    // Given: hook is initialized
    mockGetActiveAccount.mockResolvedValueOnce(null);
    const { result } = renderHook(() => useAuth());
    await act(async () => {});

    // When: login fails
    mockStartOAuth.mockRejectedValueOnce(new Error("OAuth flow cancelled"));
    await act(async () => {
      await result.current.login().catch(() => {});
    });

    // Then: error state is set
    expect(result.current.error).toBe("OAuth flow cancelled");
  });
});
