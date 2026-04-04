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

  it("should set isLoading to true during refresh", async () => {
    // Given: initial load completes
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});
    expect(result.current.isLoading).toBe(false);

    // When: refresh is called but fetch is still pending
    let resolveRefreshFetch: (value: unknown) => void;
    mockListMessages.mockReturnValueOnce(
      new Promise((resolve) => {
        resolveRefreshFetch = resolve;
      }),
    );

    let refreshPromise: Promise<void>;
    act(() => {
      refreshPromise = result.current.refresh();
    });

    // Then: isLoading is true while fetch is in progress
    expect(result.current.isLoading).toBe(true);

    // Cleanup: resolve the pending fetch
    await act(async () => {
      resolveRefreshFetch!({
        messages: [{ id: "msg2", subject: "New", from: "x@y.com", snippet: "", date: "", isUnread: false }],
        nextPageToken: null,
      });
      await refreshPromise!;
    });
  });

  it("should set isLoading to false after refresh completes", async () => {
    // Given: initial load completes
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: refresh completes successfully
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg2", subject: "New", from: "x@y.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    await act(async () => {
      await result.current.refresh();
    });

    // Then: isLoading is false
    expect(result.current.isLoading).toBe(false);
  });

  it("should set error when refresh fails", async () => {
    // Given: initial load completes
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});
    expect(result.current.error).toBeNull();

    // When: refresh fails with an error
    mockListMessages.mockRejectedValueOnce(new Error("Token expired"));
    await act(async () => {
      await result.current.refresh();
    });

    // Then: error state is set and isLoading is false
    expect(result.current.error).toBe("Token expired");
    expect(result.current.isLoading).toBe(false);
    expect(result.current.messages).toEqual([]);
  });

  it("should clear previous error when refresh is called again", async () => {
    // Given: initial load fails
    mockListMessages.mockRejectedValueOnce(new Error("Network error"));
    const { result } = renderHook(() => useMails());
    await act(async () => {});
    expect(result.current.error).toBe("Network error");

    // When: refresh is called and succeeds
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Recovered", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    await act(async () => {
      await result.current.refresh();
    });

    // Then: error is cleared
    expect(result.current.error).toBeNull();
    expect(result.current.messages).toHaveLength(1);
  });

  it("should set isLoading to false even when refresh fails", async () => {
    // Given: initial load completes
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: refresh fails
    mockListMessages.mockRejectedValueOnce(new Error("Server error"));
    await act(async () => {
      await result.current.refresh();
    });

    // Then: isLoading is false (finally block executed)
    expect(result.current.isLoading).toBe(false);
  });

  it("should handle non-Error thrown values during refresh", async () => {
    // Given: initial load completes
    mockListMessages.mockResolvedValueOnce({
      messages: [{ id: "msg1", subject: "Test", from: "a@b.com", snippet: "", date: "", isUnread: false }],
      nextPageToken: null,
    });
    const { result } = renderHook(() => useMails());
    await act(async () => {});

    // When: refresh fails with a string error (not Error instance)
    mockListMessages.mockRejectedValueOnce("API unavailable");
    await act(async () => {
      await result.current.refresh();
    });

    // Then: error is converted to string
    expect(result.current.error).toBe("API unavailable");
    expect(result.current.isLoading).toBe(false);
  });
});
