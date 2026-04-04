import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { MailScreen } from "../MailScreen";
import type {
  MessageSummary,
  MessageDetail,
  AccountInfo,
} from "../../../shared/types";

// --- Mock data ---

const mockMessages: MessageSummary[] = [
  {
    id: "msg1",
    subject: "First Email",
    from: "alice@gmail.com",
    snippet: "Hello, this is the first email content preview.",
    date: "2026-04-01T10:00:00Z",
    isUnread: true,
  },
  {
    id: "msg2",
    subject: "Second Email",
    from: "bob@gmail.com",
    snippet: "Another email with some preview text here.",
    date: "2026-04-01T09:00:00Z",
    isUnread: false,
  },
];

const mockMessageDetail: MessageDetail = {
  id: "msg1",
  subject: "First Email",
  from: "alice@gmail.com",
  date: "2026-04-01T10:00:00Z",
  body: "<p>Full email body</p>",
  contentType: "text/html",
};

const mockAccounts: AccountInfo[] = [
  { email: "user1@gmail.com", isActive: true },
  { email: "user2@gmail.com", isActive: false },
];

// --- Mock hooks ---

const mockLoadMore = vi.fn();
const mockRefresh = vi.fn();
const mockGetMessageDetail = vi.fn();
const mockSwitchAccount = vi.fn();
const mockAddAccount = vi.fn();

vi.mock("../useMails", () => ({
  useMails: () => ({
    messages: mockMessages,
    hasMore: true,
    isLoading: false,
    error: null,
    loadMore: mockLoadMore,
    refresh: mockRefresh,
    getMessageDetail: mockGetMessageDetail,
  }),
}));

vi.mock("../../accounts/useAccounts", () => ({
  useAccounts: () => ({
    accounts: mockAccounts,
    activeAccount: mockAccounts[0],
    isLoading: false,
    error: null,
    switchAccount: mockSwitchAccount,
    addAccount: mockAddAccount,
  }),
}));

vi.mock("../useNewMailNotification", () => ({
  useNewMailNotification: vi.fn(),
}));

describe("MailScreen", () => {
  beforeEach(() => {
    mockLoadMore.mockReset();
    mockRefresh.mockReset();
    mockGetMessageDetail.mockReset();
    mockSwitchAccount.mockReset();
    mockAddAccount.mockReset();
    mockGetMessageDetail.mockResolvedValue(mockMessageDetail);
    mockSwitchAccount.mockResolvedValue(undefined);
    mockRefresh.mockResolvedValue(undefined);
  });

  // --- 2-pane layout ---

  it("should render header bar, mail list, and mail detail areas", () => {
    // Given/When: rendering MailScreen
    render(<MailScreen />);

    // Then: header bar area exists (with toggle and account buttons)
    expect(
      screen.getByRole("button", { name: /toggle|サイドバー|メニュー/i }),
    ).toBeInTheDocument();

    // Then: mail list area shows messages
    expect(screen.getByText("First Email")).toBeInTheDocument();
    expect(screen.getByText("Second Email")).toBeInTheDocument();
  });

  it("should show placeholder when no message is selected", () => {
    // Given/When: rendering MailScreen without selecting any message
    render(<MailScreen />);

    // Then: a placeholder message is shown in the detail pane
    expect(
      screen.getByText(/メールを選択|select.*mail|select.*message/i),
    ).toBeInTheDocument();
  });

  // --- Message selection ---

  it("should display message detail when a message is selected", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: clicking on a message in the list
    fireEvent.click(screen.getByText("First Email"));

    // Then: message detail is fetched and displayed
    await waitFor(() => {
      expect(mockGetMessageDetail).toHaveBeenCalledWith("msg1");
    });
  });

  // --- Account switching with mail refresh ---

  it("should refresh mail list after switching account", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: opening account dropdown and switching account
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );
    fireEvent.click(screen.getByText("user2@gmail.com"));

    // Then: switchAccount is called, then refresh is called
    await waitFor(() => {
      expect(mockSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
    });
    await waitFor(() => {
      expect(mockRefresh).toHaveBeenCalledTimes(1);
    });
  });

  it("should clear selected message after switching account", async () => {
    // Given: MailScreen is rendered and a message is selected
    render(<MailScreen />);
    fireEvent.click(screen.getByText("First Email"));
    await waitFor(() => {
      expect(mockGetMessageDetail).toHaveBeenCalledWith("msg1");
    });

    // When: switching account
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );
    fireEvent.click(screen.getByText("user2@gmail.com"));

    // Then: selected message is cleared, placeholder is shown
    await waitFor(() => {
      expect(
        screen.getByText(/メールを選択|select.*mail|select.*message/i),
      ).toBeInTheDocument();
    });
  });

  it("should clear selected message even when refresh fails after switching account", async () => {
    // Given: MailScreen is rendered and a message is selected
    render(<MailScreen />);
    fireEvent.click(screen.getByText("First Email"));
    await waitFor(() => {
      expect(mockGetMessageDetail).toHaveBeenCalledWith("msg1");
    });

    // When: switching account but refresh fails
    mockRefresh.mockRejectedValueOnce(new Error("Network error"));
    fireEvent.click(
      screen.getByRole("button", { name: /アカウント|account/i }),
    );
    fireEvent.click(screen.getByText("user2@gmail.com"));

    // Then: selected message is still cleared, placeholder is shown
    await waitFor(() => {
      expect(
        screen.getByText(/メールを選択|select.*mail|select.*message/i),
      ).toBeInTheDocument();
    });
  });

  // --- Sidebar toggle ---

  it("should show mail list by default", () => {
    // Given/When: rendering MailScreen
    render(<MailScreen />);

    // Then: mail list is visible
    expect(screen.getByText("First Email")).toBeInTheDocument();
  });

  it("should hide mail list when toggle button is clicked", () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: clicking the sidebar toggle button
    fireEvent.click(
      screen.getByRole("button", { name: /toggle|サイドバー|メニュー/i }),
    );

    // Then: mail list is hidden
    expect(screen.queryByText("First Email")).not.toBeInTheDocument();
  });

  it("should show mail list again when toggle button is clicked twice", () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: clicking toggle twice
    const toggleButton = screen.getByRole("button", {
      name: /toggle|サイドバー|メニュー/i,
    });
    fireEvent.click(toggleButton);
    fireEvent.click(toggleButton);

    // Then: mail list is visible again
    expect(screen.getByText("First Email")).toBeInTheDocument();
  });

  // --- Cmd+B keyboard shortcut ---

  it("should toggle sidebar with Cmd+B keyboard shortcut", () => {
    // Given: MailScreen is rendered with sidebar visible
    render(<MailScreen />);
    expect(screen.getByText("First Email")).toBeInTheDocument();

    // When: pressing Cmd+B
    fireEvent.keyDown(document, { key: "b", metaKey: true });

    // Then: mail list is hidden
    expect(screen.queryByText("First Email")).not.toBeInTheDocument();
  });

  it("should restore sidebar with second Cmd+B press", () => {
    // Given: MailScreen is rendered and sidebar is toggled off
    render(<MailScreen />);
    fireEvent.keyDown(document, { key: "b", metaKey: true });
    expect(screen.queryByText("First Email")).not.toBeInTheDocument();

    // When: pressing Cmd+B again
    fireEvent.keyDown(document, { key: "b", metaKey: true });

    // Then: mail list is visible again
    expect(screen.getByText("First Email")).toBeInTheDocument();
  });

  it("should not toggle sidebar with just B key (without meta)", () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: pressing B without meta key
    fireEvent.keyDown(document, { key: "b", metaKey: false });

    // Then: mail list remains visible
    expect(screen.getByText("First Email")).toBeInTheDocument();
  });
});
