import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { AccountSwitcher } from "../AccountSwitcher";
import type { AccountInfo } from "../../../shared/types";

describe("AccountSwitcher", () => {
  const accounts: AccountInfo[] = [
    { email: "user1@gmail.com", isActive: true },
    { email: "user2@gmail.com", isActive: false },
  ];
  const activeAccount: AccountInfo = {
    email: "user1@gmail.com",
    isActive: true,
  };
  const mockOnSwitchAccount = vi.fn();
  const mockOnAddAccount = vi.fn();
  const mockOnLogout = vi.fn();

  beforeEach(() => {
    mockOnSwitchAccount.mockReset();
    mockOnAddAccount.mockReset();
    mockOnLogout.mockReset();
  });

  // --- Dropdown trigger ---

  it("should render an account icon as dropdown trigger", () => {
    // Given/When: rendering AccountSwitcher
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // Then: account icon button is present
    const trigger = screen.getByRole("button", {
      name: /アカウント|account/i,
    });
    expect(trigger).toBeInTheDocument();
  });

  // --- Dropdown closed by default ---

  it("should not show account list by default", () => {
    // Given/When: rendering without interaction
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // Then: account emails are not visible
    expect(screen.queryByText("user1@gmail.com")).not.toBeInTheDocument();
    expect(screen.queryByText("user2@gmail.com")).not.toBeInTheDocument();
  });

  // --- Open dropdown ---

  it("should show all accounts when trigger is clicked", () => {
    // Given: AccountSwitcher is rendered
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // When: clicking the trigger
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: all accounts are displayed
    expect(screen.getByText("user1@gmail.com")).toBeInTheDocument();
    expect(screen.getByText("user2@gmail.com")).toBeInTheDocument();
  });

  // --- Active account indicator ---

  it("should indicate the active account", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: active account has data-active attribute
    const activeItem = screen
      .getByText("user1@gmail.com")
      .closest("[data-active]");
    expect(activeItem).toHaveAttribute("data-active", "true");
  });

  // --- Switch account ---

  it("should call onSwitchAccount when clicking inactive account", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking the inactive account
    fireEvent.click(screen.getByText("user2@gmail.com"));

    // Then: onSwitchAccount is called with the correct email
    expect(mockOnSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
  });

  it("should not call onSwitchAccount when clicking active account", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking the already active account
    fireEvent.click(screen.getByText("user1@gmail.com"));

    // Then: onSwitchAccount is not called
    expect(mockOnSwitchAccount).not.toHaveBeenCalled();
  });

  // --- Close after selection ---

  it("should close dropdown after switching account", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking an inactive account
    fireEvent.click(screen.getByText("user2@gmail.com"));

    // Then: dropdown is closed
    expect(screen.queryByText("user2@gmail.com")).not.toBeInTheDocument();
  });

  // --- Add account ---

  it("should show add account button in dropdown", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: add account button is visible
    const addButton = screen.getByRole("button", {
      name: /アカウント.*追加|add.*account/i,
    });
    expect(addButton).toBeInTheDocument();
  });

  it("should call onAddAccount when clicking add button", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking add account button
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント.*追加|add.*account/i }),
    );

    // Then: onAddAccount is called
    expect(mockOnAddAccount).toHaveBeenCalledTimes(1);
  });

  // --- Logout button ---

  it("should show logout button in dropdown with red text", () => {
    // Given: dropdown is open
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: logout button is visible with red color
    const logoutButton = screen.getByRole("button", { name: /ログアウト/i });
    expect(logoutButton).toBeInTheDocument();
    expect(logoutButton).toHaveStyle({ color: "#d32f2f" });
  });

  it("should call onLogout when confirm is accepted", () => {
    // Given: dropdown is open, confirm returns true
    vi.spyOn(window, "confirm").mockReturnValueOnce(true);
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking logout button
    fireEvent.click(screen.getByRole("button", { name: /ログアウト/i }));

    // Then: confirm dialog was shown and onLogout was called
    expect(window.confirm).toHaveBeenCalledWith(
      "user1@gmail.com からログアウトしますか？",
    );
    expect(mockOnLogout).toHaveBeenCalledTimes(1);
  });

  it("should not call onLogout when confirm is cancelled", () => {
    // Given: dropdown is open, confirm returns false
    vi.spyOn(window, "confirm").mockReturnValueOnce(false);
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking logout button but cancelling
    fireEvent.click(screen.getByRole("button", { name: /ログアウト/i }));

    // Then: onLogout was not called
    expect(mockOnLogout).not.toHaveBeenCalled();
  });

  it("should close dropdown after logout confirmation", () => {
    // Given: dropdown is open, confirm returns true
    vi.spyOn(window, "confirm").mockReturnValueOnce(true);
    render(
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking logout button and confirming
    fireEvent.click(screen.getByRole("button", { name: /ログアウト/i }));

    // Then: dropdown is closed
    expect(screen.queryByRole("button", { name: /ログアウト/i })).not.toBeInTheDocument();
  });
});
