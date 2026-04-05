import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { HeaderBar } from "../components/header-bar";
import type { AccountInfo } from "../../../shared/types";

describe("HeaderBar", () => {
  const accounts: AccountInfo[] = [
    { email: "user1@gmail.com", isActive: true },
    { email: "user2@gmail.com", isActive: false },
    { email: "user3@outlook.com", isActive: false },
  ];
  const activeAccount: AccountInfo = {
    email: "user1@gmail.com",
    isActive: true,
  };
  const mockOnToggleSidebar = vi.fn();
  const mockOnSwitchAccount = vi.fn();
  const mockOnAddAccount = vi.fn();
  const mockOnLogout = vi.fn();

  beforeEach(() => {
    mockOnToggleSidebar.mockReset();
    mockOnSwitchAccount.mockReset();
    mockOnAddAccount.mockReset();
    mockOnLogout.mockReset();
  });

  // --- Toggle button ---

  it("should render a sidebar toggle button", () => {
    // Given/When: rendering HeaderBar
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // Then: toggle button is present
    const toggleButton = screen.getByRole("button", {
      name: /toggle|サイドバー|メニュー/i,
    });
    expect(toggleButton).toBeInTheDocument();
  });

  it("should call onToggleSidebar when toggle button is clicked", () => {
    // Given: HeaderBar is rendered
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // When: clicking the toggle button
    fireEvent.click(
      screen.getByRole("button", { name: /toggle|サイドバー|メニュー/i }),
    );

    // Then: onToggleSidebar is called
    expect(mockOnToggleSidebar).toHaveBeenCalledTimes(1);
  });

  // --- Account icon ---

  it("should render an account icon trigger", () => {
    // Given/When: rendering HeaderBar
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // Then: account icon button is present
    const accountButton = screen.getByRole("button", {
      name: /アカウント|account/i,
    });
    expect(accountButton).toBeInTheDocument();
  });

  // --- Dropdown ---

  it("should not show account dropdown by default", () => {
    // Given/When: rendering HeaderBar without interaction
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // Then: account list is not visible
    expect(screen.queryByText("user2@gmail.com")).not.toBeInTheDocument();
  });

  it("should show account dropdown when account icon is clicked", () => {
    // Given: HeaderBar is rendered
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );

    // When: clicking the account icon
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: all accounts are displayed in the dropdown
    expect(screen.getByText("user1@gmail.com")).toBeInTheDocument();
    expect(screen.getByText("user2@gmail.com")).toBeInTheDocument();
    expect(screen.getByText("user3@outlook.com")).toBeInTheDocument();
  });

  it("should close dropdown when account icon is clicked again", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    const accountButton = screen.getByRole("button", {
      name: /アカウント|account/i,
    });
    fireEvent.click(accountButton);

    // When: clicking the account icon again
    fireEvent.click(accountButton);

    // Then: dropdown is closed
    expect(screen.queryByText("user2@gmail.com")).not.toBeInTheDocument();
  });

  // --- Account switching ---

  it("should call onSwitchAccount when clicking an inactive account in dropdown", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
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

    // Then: onSwitchAccount is called with the email
    expect(mockOnSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
  });

  it("should not call onSwitchAccount when clicking the active account", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // When: clicking the active account
    fireEvent.click(screen.getByText("user1@gmail.com"));

    // Then: onSwitchAccount is not called
    expect(mockOnSwitchAccount).not.toHaveBeenCalled();
  });

  // --- Active account indicator in dropdown ---

  it("should indicate the active account in dropdown", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
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

  // --- Add account button in dropdown ---

  it("should show add account button in dropdown", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
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

  it("should call onAddAccount when clicking add account button", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
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

  // --- Logout button in dropdown ---

  it("should show logout button in dropdown", () => {
    // Given: HeaderBar is rendered and dropdown is open
    render(
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={mockOnToggleSidebar}
        onSwitchAccount={mockOnSwitchAccount}
        onAddAccount={mockOnAddAccount}
        onLogout={mockOnLogout}
      />,
    );
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );

    // Then: logout button is visible
    expect(
      screen.getByRole("button", { name: /ログアウト/i }),
    ).toBeInTheDocument();
  });
});
