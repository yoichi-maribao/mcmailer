import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MailList } from "../MailList";
import type { MessageSummary } from "../../../shared/types";

describe("MailList", () => {
  const mockMessages: MessageSummary[] = [
    {
      id: "msg1",
      subject: "Important Update",
      from: "boss@company.com",
      snippet: "Please review the attached document...",
      date: "2026-04-01T10:00:00Z",
      isUnread: true,
    },
    {
      id: "msg2",
      subject: "Newsletter #42",
      from: "news@example.com",
      snippet: "This week in tech...",
      date: "2026-03-31T08:00:00Z",
      isUnread: false,
    },
    {
      id: "msg3",
      subject: "Meeting Notes",
      from: "colleague@company.com",
      snippet: "Here are the notes from today's...",
      date: "2026-03-30T15:00:00Z",
      isUnread: false,
    },
  ];

  const mockOnSelect = vi.fn();
  const mockOnLoadMore = vi.fn();
  const mockOnRefresh = vi.fn();

  beforeEach(() => {
    mockOnSelect.mockReset();
    mockOnLoadMore.mockReset();
    mockOnRefresh.mockReset();
  });

  // --- Rendering ---

  it("should render all message items", () => {
    // Given: a list of messages

    // When: rendering the mail list
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: all messages are displayed
    expect(screen.getByText("Important Update")).toBeInTheDocument();
    expect(screen.getByText("Newsletter #42")).toBeInTheDocument();
    expect(screen.getByText("Meeting Notes")).toBeInTheDocument();
  });

  it("should display snippet preview for each message", () => {
    // Given/When: rendering the mail list
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: snippets are shown
    expect(
      screen.getByText(/Please review the attached document/),
    ).toBeInTheDocument();
  });

  // --- Unread indicator ---

  it("should visually distinguish unread messages", () => {
    // Given/When: rendering with unread messages
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: unread message has unread indicator
    const unreadItem = screen.getByText("Important Update").closest("[data-unread]");
    expect(unreadItem).toHaveAttribute("data-unread", "true");
  });

  // --- Selection ---

  it("should call onSelect when clicking a message", () => {
    // Given: rendering the mail list
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // When: clicking a message
    fireEvent.click(screen.getByText("Newsletter #42"));

    // Then: onSelect is called with the message ID
    expect(mockOnSelect).toHaveBeenCalledWith("msg2");
  });

  it("should highlight the selected message", () => {
    // Given/When: rendering with selectedId
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId="msg2"
      />,
    );

    // Then: the selected message has selected indicator
    const selectedItem = screen
      .getByText("Newsletter #42")
      .closest("[data-selected]");
    expect(selectedItem).toHaveAttribute("data-selected", "true");
  });

  it("should not highlight non-selected messages", () => {
    // Given/When: rendering with selectedId
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId="msg2"
      />,
    );

    // Then: non-selected messages do not have selected indicator
    const nonSelectedItem = screen
      .getByText("Important Update")
      .closest("[data-selected]");
    expect(nonSelectedItem).toHaveAttribute("data-selected", "false");
  });

  // --- Infinite scroll (IntersectionObserver) ---

  it("should render a sentinel element when hasMore is true", () => {
    // Given/When: rendering with hasMore=true
    const { container } = render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={true}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: a sentinel element exists for IntersectionObserver
    const sentinel = container.querySelector("[data-testid='scroll-sentinel']");
    expect(sentinel).not.toBeNull();
  });

  it("should not render sentinel element when hasMore is false", () => {
    // Given/When: rendering with hasMore=false
    const { container } = render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: no sentinel element
    const sentinel = container.querySelector("[data-testid='scroll-sentinel']");
    expect(sentinel).toBeNull();
  });

  describe("IntersectionObserver integration", () => {
    afterEach(() => {
      vi.unstubAllGlobals();
    });

    it("should call onLoadMore when sentinel becomes visible", () => {
      // Given: IntersectionObserver mock
      let observerCallback: IntersectionObserverCallback;
      const mockObserve = vi.fn();
      const mockDisconnect = vi.fn();

      vi.stubGlobal(
        "IntersectionObserver",
        class {
          constructor(callback: IntersectionObserverCallback) {
            observerCallback = callback;
          }
          observe = mockObserve;
          disconnect = mockDisconnect;
          unobserve = vi.fn();
        },
      );

      render(
        <MailList
          messages={mockMessages}
          onSelect={mockOnSelect}
          onLoadMore={mockOnLoadMore}
          onRefresh={mockOnRefresh}
          hasMore={true}
          isLoading={false}
          isLoadingMore={false}
          selectedId={null}
        />,
      );

      // When: sentinel becomes visible (intersecting)
      observerCallback!(
        [{ isIntersecting: true } as IntersectionObserverEntry],
        {} as IntersectionObserver,
      );

      // Then: onLoadMore is called
      expect(mockOnLoadMore).toHaveBeenCalledTimes(1);
    });

    it("should not call onLoadMore when sentinel is not intersecting", () => {
      // Given: IntersectionObserver mock
      let observerCallback: IntersectionObserverCallback;
      const mockObserve = vi.fn();
      const mockDisconnect = vi.fn();

      vi.stubGlobal(
        "IntersectionObserver",
        class {
          constructor(callback: IntersectionObserverCallback) {
            observerCallback = callback;
          }
          observe = mockObserve;
          disconnect = mockDisconnect;
          unobserve = vi.fn();
        },
      );

      render(
        <MailList
          messages={mockMessages}
          onSelect={mockOnSelect}
          onLoadMore={mockOnLoadMore}
          onRefresh={mockOnRefresh}
          hasMore={true}
          isLoading={false}
          isLoadingMore={false}
          selectedId={null}
        />,
      );

      // When: sentinel is not intersecting
      observerCallback!(
        [{ isIntersecting: false } as IntersectionObserverEntry],
        {} as IntersectionObserver,
      );

      // Then: onLoadMore is not called
      expect(mockOnLoadMore).not.toHaveBeenCalled();
    });
  });

  // --- Empty state ---

  it("should show empty state when no messages", () => {
    // Given/When: rendering with empty message list
    render(
      <MailList
        messages={[]}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        onRefresh={mockOnRefresh}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
      />,
    );

    // Then: empty state message is shown
    expect(
      screen.getByText(/メールがありません|no messages|inbox is empty/i),
    ).toBeInTheDocument();
  });

  // --- Loading state ---

  it("should show loading indicator when isLoading is true", () => {
    // Given/When: rendering in loading state
    render(
      <MailList
        messages={[]}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        hasMore={false}
        isLoading={true}
        isLoadingMore={false}
        selectedId={null}
        onRefresh={mockOnRefresh}
      />,
    );

    // Then: loading indicator is visible
    expect(screen.getByRole("progressbar")).toBeInTheDocument();
  });

  // --- Reload button ---

  it("should render a reload button", () => {
    // Given/When: rendering the mail list with messages
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
        onRefresh={mockOnRefresh}
      />,
    );

    // Then: a reload button is present
    expect(
      screen.getByRole("button", { name: /リロード|reload|refresh|再取得/i }),
    ).toBeInTheDocument();
  });

  it("should call onRefresh when reload button is clicked", () => {
    // Given: rendering the mail list
    render(
      <MailList
        messages={mockMessages}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        hasMore={false}
        isLoading={false}
        isLoadingMore={false}
        selectedId={null}
        onRefresh={mockOnRefresh}
      />,
    );

    // When: clicking the reload button
    fireEvent.click(
      screen.getByRole("button", { name: /リロード|reload|refresh|再取得/i }),
    );

    // Then: onRefresh is called
    expect(mockOnRefresh).toHaveBeenCalledTimes(1);
  });

  it("should disable reload button when isLoading is true", () => {
    // Given/When: rendering in loading state with onRefresh
    render(
      <MailList
        messages={[]}
        onSelect={mockOnSelect}
        onLoadMore={mockOnLoadMore}
        hasMore={false}
        isLoading={true}
        isLoadingMore={false}
        selectedId={null}
        onRefresh={mockOnRefresh}
      />,
    );

    // Then: reload button is not rendered during loading
    // (loading indicator replaces the entire list via early return)
    const reloadButton = screen.queryByRole("button", {
      name: /リロード|reload|refresh|再取得/i,
    });
    expect(reloadButton).not.toBeInTheDocument();
  });
});
