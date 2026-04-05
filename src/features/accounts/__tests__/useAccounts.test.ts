import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";

const mockListAccounts = vi.fn();
const mockSwitchAccount = vi.fn();
const mockStartOAuth = vi.fn();
const mockRemoveAccount = vi.fn();

vi.mock("../../../shared/commands", () => ({
  listAccounts: (...args: unknown[]) => mockListAccounts(...args),
  switchAccount: (...args: unknown[]) => mockSwitchAccount(...args),
  startOAuth: (...args: unknown[]) => mockStartOAuth(...args),
  removeAccount: (...args: unknown[]) => mockRemoveAccount(...args),
}));

import { useAccounts } from "../useAccounts";

describe("useAccounts", () => {
  beforeEach(() => {
    mockListAccounts.mockReset();
    mockSwitchAccount.mockReset();
    mockStartOAuth.mockReset();
    mockRemoveAccount.mockReset();
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

  // --- removeCurrentAccount ---

  it("should remove current account and switch to remaining account", async () => {
    // Given: two accounts loaded, user1 is active
    const initialAccounts = [
      { email: "user1@gmail.com", isActive: true },
      { email: "user2@gmail.com", isActive: false },
    ];
    mockListAccounts.mockResolvedValueOnce(initialAccounts);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: removing the current account
    mockRemoveAccount.mockResolvedValueOnce(undefined);
    // After remove, listAccounts returns only user2
    const afterRemove = [{ email: "user2@gmail.com", isActive: false }];
    mockListAccounts.mockResolvedValueOnce(afterRemove);
    // After switchAccount, listAccounts returns user2 as active
    mockSwitchAccount.mockResolvedValueOnce(undefined);
    const afterSwitch = [{ email: "user2@gmail.com", isActive: true }];
    mockListAccounts.mockResolvedValueOnce(afterSwitch);

    let removeResult: { remaining: unknown[] } | undefined;
    await act(async () => {
      removeResult = await result.current.removeCurrentAccount();
    });

    // Then: removeAccount was called with the active email
    expect(mockRemoveAccount).toHaveBeenCalledWith("user1@gmail.com");
    // Then: switchAccount was called with the first remaining account
    expect(mockSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
    // Then: remaining accounts are returned
    expect(removeResult!.remaining).toEqual(afterSwitch);
    // Then: accounts state is updated
    expect(result.current.accounts).toEqual(afterSwitch);
  });

  it("should remove current account and return empty when last account", async () => {
    // Given: one account loaded
    const initialAccounts = [{ email: "user1@gmail.com", isActive: true }];
    mockListAccounts.mockResolvedValueOnce(initialAccounts);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: removing the last account
    mockRemoveAccount.mockResolvedValueOnce(undefined);
    mockListAccounts.mockResolvedValueOnce([]);

    let removeResult: { remaining: unknown[] } | undefined;
    await act(async () => {
      removeResult = await result.current.removeCurrentAccount();
    });

    // Then: removeAccount was called
    expect(mockRemoveAccount).toHaveBeenCalledWith("user1@gmail.com");
    // Then: switchAccount was NOT called (no remaining accounts)
    expect(mockSwitchAccount).not.toHaveBeenCalled();
    // Then: empty remaining is returned
    expect(removeResult!.remaining).toEqual([]);
    // Then: accounts state is empty
    expect(result.current.accounts).toEqual([]);
  });

  it("should throw when removeCurrentAccount is called with no active account", async () => {
    // Given: no accounts loaded
    mockListAccounts.mockResolvedValueOnce([]);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When/Then: removeCurrentAccount throws
    await act(async () => {
      await expect(result.current.removeCurrentAccount()).rejects.toThrow(
        "No active account to remove",
      );
    });
  });

  it("should set error when removeCurrentAccount fails", async () => {
    // Given: one account loaded
    mockListAccounts.mockResolvedValueOnce([
      { email: "user1@gmail.com", isActive: true },
    ]);
    const { result } = renderHook(() => useAccounts());
    await act(async () => {});

    // When: removeAccount backend call fails
    mockRemoveAccount.mockRejectedValueOnce(new Error("Remove failed"));
    await act(async () => {
      await result.current.removeCurrentAccount().catch(() => {});
    });

    // Then: error is set
    expect(result.current.error).toBe("Remove failed");
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
