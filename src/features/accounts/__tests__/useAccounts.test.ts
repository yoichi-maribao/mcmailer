import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";

const mockListAccounts = vi.fn();
const mockSwitchAccount = vi.fn();
const mockStartOAuth = vi.fn();

vi.mock("../../../shared/commands", () => ({
  listAccounts: (...args: unknown[]) => mockListAccounts(...args),
  switchAccount: (...args: unknown[]) => mockSwitchAccount(...args),
  startOAuth: (...args: unknown[]) => mockStartOAuth(...args),
}));

import { useAccounts } from "../useAccounts";

describe("useAccounts", () => {
  beforeEach(() => {
    mockListAccounts.mockReset();
    mockSwitchAccount.mockReset();
    mockStartOAuth.mockReset();
  });

  // --- Initial loading ---

  it("should load accounts on mount", async () => {
    // Given: backend returns account list
    const accounts = [
      { email: "user1@gmail.com", isActive: true },
      { email: "user2@gmail.com", isActive: false },
    ];
    mockListAccounts.mockResolvedValueOnce(accounts);

    // When: rendering the hook
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // Then: accounts are loaded
    expect(mockListAccounts).toHaveBeenCalled();
    expect(result.current.accounts).toEqual(accounts);
  });

  it("should identify the active account", async () => {
    // Given: backend returns accounts with one active
    const accounts = [
      { email: "user1@gmail.com", isActive: true },
      { email: "user2@gmail.com", isActive: false },
    ];
    mockListAccounts.mockResolvedValueOnce(accounts);

    // When: rendering the hook
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // Then: active account is identified
    expect(result.current.activeAccount).toEqual({
      email: "user1@gmail.com",
      isActive: true,
    });
  });

  it("should have null active account when no accounts exist", async () => {
    // Given: backend returns empty account list
    mockListAccounts.mockResolvedValueOnce([]);

    // When: rendering the hook
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // Then: active account is null
    expect(result.current.activeAccount).toBeNull();
  });

  // --- switchAccount ---

  it("should switch account and reload accounts", async () => {
    // Given: initial accounts loaded
    const initialAccounts = [
      { email: "user1@gmail.com", isActive: true },
      { email: "user2@gmail.com", isActive: false },
    ];
    mockListAccounts.mockResolvedValueOnce(initialAccounts);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: switching to user2
    const updatedAccounts = [
      { email: "user1@gmail.com", isActive: false },
      { email: "user2@gmail.com", isActive: true },
    ];
    mockSwitchAccount.mockResolvedValueOnce(undefined);
    mockListAccounts.mockResolvedValueOnce(updatedAccounts);
    await act(async () => {
      await result.current.switchAccount("user2@gmail.com");
    });

    // Then: switchAccount was called and accounts reloaded
    expect(mockSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
    expect(result.current.activeAccount).toEqual({
      email: "user2@gmail.com",
      isActive: true,
    });
  });

  // --- addAccount ---

  it("should call startOAuth to add new account", async () => {
    // Given: initial accounts loaded
    mockListAccounts.mockResolvedValueOnce([
      { email: "user1@gmail.com", isActive: true },
    ]);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: adding a new account
    mockStartOAuth.mockResolvedValueOnce(undefined);
    await act(async () => {
      await result.current.addAccount();
    });

    // Then: startOAuth was called
    expect(mockStartOAuth).toHaveBeenCalled();
  });

  // --- Error handling ---

  it("should set error when switch fails", async () => {
    // Given: initial accounts loaded
    mockListAccounts.mockResolvedValueOnce([
      { email: "user1@gmail.com", isActive: true },
    ]);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: switching fails
    mockSwitchAccount.mockRejectedValueOnce(new Error("Switch failed"));
    await act(async () => {
      await result.current.switchAccount("bad@gmail.com").catch(() => {});
    });

    // Then: error is set
    expect(result.current.error).toBe("Switch failed");
  });
});
