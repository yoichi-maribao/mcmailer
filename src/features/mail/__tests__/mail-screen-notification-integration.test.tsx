import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { MailScreen } from "../MailScreen";
import type {
  MessageSummary,
  MessageDetail as MessageDetailType,
  AccountInfo,
} from "../../../shared/types";
import type { NewMailEvent, NavigateToMailEvent } from "../../../shared/events";

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

const mockMessageDetail: MessageDetailType = {
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

// --- Mock useNewMailNotification ---

let capturedOnNewMail: ((event: NewMailEvent) => void) | null = null;
let capturedOnNavigateToMail: ((event: NavigateToMailEvent) => void) | null =
  null;

vi.mock("../useNewMailNotification", () => ({
  useNewMailNotification: ({
    onNewMail,
    onNavigateToMail,
  }: {
    onNewMail: (event: NewMailEvent) => void;
    onNavigateToMail: (event: NavigateToMailEvent) => void;
  }) => {
    capturedOnNewMail = onNewMail;
    capturedOnNavigateToMail = onNavigateToMail;
  },
}));

describe("MailScreen notification integration", () => {
  beforeEach(() => {
    mockLoadMore.mockReset();
    mockRefresh.mockReset();
    mockGetMessageDetail.mockReset();
    mockSwitchAccount.mockReset();
    mockAddAccount.mockReset();
    mockGetMessageDetail.mockResolvedValue(mockMessageDetail);
    mockSwitchAccount.mockResolvedValue(undefined);
    mockRefresh.mockResolvedValue(undefined);
    capturedOnNewMail = null;
    capturedOnNavigateToMail = null;
  });

  // --- useNewMailNotification hook is wired ---

  it("should register useNewMailNotification hook with callbacks", () => {
    // Given/When: rendering MailScreen
    render(<MailScreen />);

    // Then: useNewMailNotification receives callback functions
    expect(capturedOnNewMail).toBeTypeOf("function");
    expect(capturedOnNavigateToMail).toBeTypeOf("function");
  });

  // --- onNewMail triggers refresh ---

  it("should call refresh when new-mail-received event fires", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: a new mail event arrives
    capturedOnNewMail!({
      accountEmail: "user1@gmail.com",
      messageId: "msg_new_1",
      subject: "New Message",
      from: "sender@example.com",
    });

    // Then: mail list is refreshed
    await waitFor(() => {
      expect(mockRefresh).toHaveBeenCalledTimes(1);
    });
  });

  it("should call refresh for each new-mail-received event", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: multiple new mail events arrive
    capturedOnNewMail!({
      accountEmail: "user1@gmail.com",
      messageId: "msg_new_1",
      subject: "First",
      from: "a@b.com",
    });
    capturedOnNewMail!({
      accountEmail: "user1@gmail.com",
      messageId: "msg_new_2",
      subject: "Second",
      from: "c@d.com",
    });

    // Then: refresh is called for each event
    await waitFor(() => {
      expect(mockRefresh).toHaveBeenCalledTimes(2);
    });
  });

  // --- onNavigateToMail triggers account switch and message selection ---

  it("should switch account and select message when navigate-to-mail event fires", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: a navigate-to-mail event arrives for a different account
    capturedOnNavigateToMail!({
      accountEmail: "user2@gmail.com",
      messageId: "msg_target",
    });

    // Then: account is switched
    await waitFor(() => {
      expect(mockSwitchAccount).toHaveBeenCalledWith("user2@gmail.com");
    });
  });

  it("should select the target message when navigate-to-mail event fires", async () => {
    // Given: MailScreen is rendered
    mockGetMessageDetail.mockResolvedValue({
      id: "msg_target",
      subject: "Target Message",
      from: "sender@example.com",
      date: "2026-04-01T10:00:00Z",
      body: "<p>Target body</p>",
      contentType: "text/html",
    });
    render(<MailScreen />);

    // When: a navigate-to-mail event arrives
    capturedOnNavigateToMail!({
      accountEmail: "user1@gmail.com",
      messageId: "msg_target",
    });

    // Then: the target message detail is fetched
    await waitFor(() => {
      expect(mockGetMessageDetail).toHaveBeenCalledWith("msg_target");
    });
  });

  // --- Event from different account than active ---

  it("should handle new-mail event from non-active account without error", async () => {
    // Given: MailScreen is rendered with user1 active
    render(<MailScreen />);

    // When: a new mail event arrives from a different account
    capturedOnNewMail!({
      accountEmail: "user2@gmail.com",
      messageId: "msg_other",
      subject: "Other Account Mail",
      from: "someone@example.com",
    });

    // Then: refresh is called (no crash)
    await waitFor(() => {
      expect(mockRefresh).toHaveBeenCalled();
    });
  });

  // --- Existing functionality preserved after notification integration ---

  it("should still render mail list after notification hook is wired", () => {
    // Given/When: rendering MailScreen with notification hook active
    render(<MailScreen />);

    // Then: existing mail list is still rendered
    expect(screen.getByText("First Email")).toBeInTheDocument();
    expect(screen.getByText("Second Email")).toBeInTheDocument();
  });

  it("should still support manual message selection after notification hook is wired", async () => {
    // Given: MailScreen is rendered
    render(<MailScreen />);

    // When: clicking on a message manually
    fireEvent.click(screen.getByText("First Email"));

    // Then: message detail is fetched as before
    await waitFor(() => {
      expect(mockGetMessageDetail).toHaveBeenCalledWith("msg1");
    });
  });
});
