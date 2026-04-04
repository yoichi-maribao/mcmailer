import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";

const mockListMessages = vi.fn();
const mockGetMessage = vi.fn();

vi.mock("../../../shared/commands", () => ({
  listMessages: (...args: unknown[]) => mockListMessages(...args),
  getMessage: (...args: unknown[]) => mockGetMessage(...args),
}));

import { useMails } from "../useMails";

describe("useMails", () => {
  beforeEach(() => {
    mockListMessages.mockReset();
    mockGetMessage.mockReset();
  });

  // --- Initial loading ---

  it("should load inbox messages on mount", async () => {
    // Given: backend returns message list
    const mockResponse = {
      messages: [
        {
          id: "msg1",
          subject: "Welcome",
          from: "admin@gmail.com",
          snippet: "Welcome to...",
          date: "2026-01-01T00:00:00Z",
          isUnread: true,
        },
        {
          id: "msg2",
          subject: "Update",
          from: "noreply@gmail.com",
          snippet: "Your account...",
          date: "2026-01-02T00:00:00Z",
          isUnread: false,
        },
      ],
      nextPageToken: "page2token",
    };
    mockListMessages.mockResolvedValueOnce(mockResponse);

    // When: rendering the hook
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // Then: messages are loaded
    expect(mockListMessages).toHaveBeenCalledWith(null);
    expect(result.current.messages).toHaveLength(2);
    expect(result.current.messages[0].id).toBe("msg1");
    expect(result.current.hasMore).toBe(true);
  });

  it("should set hasMore to false when no nextPageToken", async () => {
    // Given: backend returns message list without next page token
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Only", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });

    // When: rendering the hook
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // Then: hasMore is false
    expect(result.current.hasMore).toBe(false);
  });

  it("should start with loading true", () => {
    // Given: listMessages is pending
    mockListMessages.mockReturnValue(new Promise(() => {}));

    // When: rendering the hook
    const { result } = renderHook(() => useMails());

    // Then: loading is true
    expect(result.current.isLoading).toBe(true);
  });

  // --- loadMore ---

  it("should append messages when loading more", async () => {
    // Given: first page loaded
    const firstPage = {
      messages: [
        { id: "msg1", subject: "First", from: "a@b.com", snippet: "", date: "", isUnread: false },
      ],
      nextPageToken: "page2",
    };
    mockListMessages.mockResolvedValueOnce(firstPage);
    const { result } = renderHook(() => useMails());
    await act(async () => {});
    expect(result.current.messages).toHaveLength(1);

    // When: loading more
    const secondPage = {
      messages: [
        { id: "msg2", subject: "Second", from: "c@d.com", snippet: "", date: "", isUnread: false },
      ],
      nextPageToken: null,
    };
    mockListMessages.mockResolvedValueOnce(secondPage);
    await act(async () => {
      await result.current.loadMore();
    });

    // Then: messages from both pages are present
    expect(result.current.messages).toHaveLength(2);
    expect(result.current.messages[0].id).toBe("msg1");
    expect(result.current.messages[1].id).toBe("msg2");
    expect(result.current.hasMore).toBe(false);
  });

  it("should not load more when no next page", async () => {
    // Given: last page loaded (no nextPageToken)
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Only", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: attempting to load more
    mockListMessages.mockClear();
    await act(async () => {
      await result.current.loadMore();
    });

    // Then: no additional API call is made
    expect(mockListMessages).not.toHaveBeenCalled();
  });

  // --- getMessageDetail ---

  it("should fetch and return message detail", async () => {
    // Given: messages are loaded
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: fetching message detail
    const detail = {
      id: "msg1",
      subject: "Test",
      from: "a@b.com",
      date: "2026-01-01",
      body: "<p>Hello world</p>",
      contentType: "text/html",
    };
    mockGetMessage.mockResolvedValueOnce(detail);

    let messageDetail: typeof detail | undefined;
    await act(async () => {
      messageDetail = await result.current.getMessageDetail("msg1");
    });

    // Then: correct message detail is returned
    expect(mockGetMessage).toHaveBeenCalledWith("msg1");
    expect(messageDetail).toEqual(detail);
  });

  // --- Error handling ---

  it("should set error when loading messages fails", async () => {
    // Given: backend returns error
    mockListMessages.mockRejectedValueOnce(new Error("Token expired"));

    // When: rendering the hook
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // Then: error is set
    expect(result.current.error).toBe("Token expired");
    expect(result.current.messages).toEqual([]);
  });

  // --- refresh ---

  it("should clear messages and reload on refresh", async () => {
    // Given: messages are loaded
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Old", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: refreshing
    const newMessages = {
      messages: [{ id: "msg3", subject: "New", from: "x@y.com", snippet: "", date: "", isUnread: true }],
      nextPageToken: null,
    };
    mockListMessages.mockResolvedValueOnce(newMessages);
    await act(async () => {
      await result.current.refresh();
    });

    // Then: old messages are replaced with new ones
    expect(result.current.messages).toHaveLength(1);
    expect(result.current.messages[0].id).toBe("msg3");
  });
});
