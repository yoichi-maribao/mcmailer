import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";

const mockListen = vi.fn();

vi.mock("@tauri-apps/api/event", () => ({
  listen: (...args: unknown[]) => mockListen(...args),
}));

const mockSwitchAccount = vi.fn();

vi.mock("../../../shared/commands", () => ({
  switchAccount: (...args: unknown[]) => mockSwitchAccount(...args),
}));

import { useNewMailNotification } from "../useNewMailNotification";

describe("useNewMailNotification", () => {
  let unlistenNewMail: ReturnType<typeof vi.fn>;
  let unlistenNavigate: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    mockListen.mockReset();
    mockSwitchAccount.mockReset();

    unlistenNewMail = vi.fn();
    unlistenNavigate = vi.fn();

    // listen is called twice: once for new-mail-received, once for navigate-to-mail
    mockListen
      .mockResolvedValueOnce(unlistenNewMail)
      .mockResolvedValueOnce(unlistenNavigate);
  });

  // --- Event listener setup ---

  it("should register listeners for new-mail-received and navigate-to-mail events", async () => {
    // Given: callbacks provided
    const onNewMail = vi.fn();
    const onNavigateToMail = vi.fn();

    // When: rendering the hook
    renderHook(() =>
      useNewMailNotification({ onNewMail, onNavigateToMail }),
    );
    await act(async () => {});

    // Then: two event listeners are registered
    expect(mockListen).toHaveBeenCalledTimes(2);
    expect(mockListen).toHaveBeenCalledWith(
      "new-mail-received",
      expect.any(Function),
    );
    expect(mockListen).toHaveBeenCalledWith(
      "navigate-to-mail",
      expect.any(Function),
    );
  });

  // --- Event listener cleanup ---

  it("should unregister listeners on unmount", async () => {
    // Given: hook is mounted with listeners
    const { unmount } = renderHook(() =>
      useNewMailNotification({
        onNewMail: vi.fn(),
        onNavigateToMail: vi.fn(),
      }),
    );
    await act(async () => {});

    // When: unmounting
    unmount();

    // Then: both unlisten functions are called
    expect(unlistenNewMail).toHaveBeenCalled();
    expect(unlistenNavigate).toHaveBeenCalled();
  });

  // --- new-mail-received event handling ---

  it("should call onNewMail when new-mail-received event fires", async () => {
    // Given: hook mounted with onNewMail callback
    const onNewMail = vi.fn();
    renderHook(() =>
      useNewMailNotification({
        onNewMail,
        onNavigateToMail: vi.fn(),
      }),
    );
    await act(async () => {});

    // When: new-mail-received event fires
    const newMailHandler = mockListen.mock.calls.find(
      (call) => call[0] === "new-mail-received",
    )?.[1];
    const eventPayload = {
      payload: {
        accountEmail: "user@gmail.com",
        messageId: "msg_new_1",
        subject: "Hello",
        from: "sender@example.com",
      },
    };
    await act(async () => {
      newMailHandler(eventPayload);
    });

    // Then: onNewMail is called with the event payload
    expect(onNewMail).toHaveBeenCalledWith({
      accountEmail: "user@gmail.com",
      messageId: "msg_new_1",
      subject: "Hello",
      from: "sender@example.com",
    });
  });

  it("should call onNewMail for each new-mail-received event independently", async () => {
    // Given: hook mounted with onNewMail callback
    const onNewMail = vi.fn();
    renderHook(() =>
      useNewMailNotification({
        onNewMail,
        onNavigateToMail: vi.fn(),
      }),
    );
    await act(async () => {});

    // When: multiple new-mail events fire
    const newMailHandler = mockListen.mock.calls.find(
      (call) => call[0] === "new-mail-received",
    )?.[1];

    await act(async () => {
      newMailHandler({
        payload: {
          accountEmail: "user@gmail.com",
          messageId: "msg_1",
          subject: "First",
          from: "a@b.com",
        },
      });
      newMailHandler({
        payload: {
          accountEmail: "user@gmail.com",
          messageId: "msg_2",
          subject: "Second",
          from: "c@d.com",
        },
      });
    });

    // Then: onNewMail is called for each event
    expect(onNewMail).toHaveBeenCalledTimes(2);
  });

  // --- navigate-to-mail event handling ---

  it("should call onNavigateToMail when navigate-to-mail event fires", async () => {
    // Given: hook mounted with onNavigateToMail callback
    const onNavigateToMail = vi.fn();
    renderHook(() =>
      useNewMailNotification({
        onNewMail: vi.fn(),
        onNavigateToMail,
      }),
    );
    await act(async () => {});

    // When: navigate-to-mail event fires
    const navigateHandler = mockListen.mock.calls.find(
      (call) => call[0] === "navigate-to-mail",
    )?.[1];
    const eventPayload = {
      payload: {
        accountEmail: "user@gmail.com",
        messageId: "msg_clicked",
      },
    };
    await act(async () => {
      navigateHandler(eventPayload);
    });

    // Then: onNavigateToMail is called with the event payload
    expect(onNavigateToMail).toHaveBeenCalledWith({
      accountEmail: "user@gmail.com",
      messageId: "msg_clicked",
    });
  });

  // --- Event from different accounts ---

  it("should pass account email from event for multi-account support", async () => {
    // Given: hook mounted
    const onNewMail = vi.fn();
    renderHook(() =>
      useNewMailNotification({
        onNewMail,
        onNavigateToMail: vi.fn(),
      }),
    );
    await act(async () => {});

    // When: events from different accounts arrive
    const newMailHandler = mockListen.mock.calls.find(
      (call) => call[0] === "new-mail-received",
    )?.[1];

    await act(async () => {
      newMailHandler({
        payload: {
          accountEmail: "alice@gmail.com",
          messageId: "msg_a1",
          subject: "From Alice's inbox",
          from: "sender@example.com",
        },
      });
      newMailHandler({
        payload: {
          accountEmail: "bob@gmail.com",
          messageId: "msg_b1",
          subject: "From Bob's inbox",
          from: "another@example.com",
        },
      });
    });

    // Then: each event includes its respective account email
    expect(onNewMail).toHaveBeenCalledTimes(2);
    expect(onNewMail.mock.calls[0][0].accountEmail).toBe("alice@gmail.com");
    expect(onNewMail.mock.calls[1][0].accountEmail).toBe("bob@gmail.com");
  });

  // --- Listener cleanup on early unmount (race condition) ---

  it("should clean up listeners even if unmount happens before listen resolves", async () => {
    // Given: listen returns promises that haven't resolved yet
    const unlistenNewMail2 = vi.fn();
    const unlistenNavigate2 = vi.fn();
    let resolveNewMail!: (fn: () => void) => void;
    let resolveNavigate!: (fn: () => void) => void;

    mockListen.mockReset();
    mockListen
      .mockReturnValueOnce(new Promise<() => void>((resolve) => { resolveNewMail = resolve; }))
      .mockReturnValueOnce(new Promise<() => void>((resolve) => { resolveNavigate = resolve; }));

    const { unmount } = renderHook(() =>
      useNewMailNotification({
        onNewMail: vi.fn(),
        onNavigateToMail: vi.fn(),
      }),
    );

    // When: unmount happens before listen promises resolve
    unmount();

    // Then: when promises resolve, unlisten functions are called immediately
    await act(async () => {
      resolveNewMail(unlistenNewMail2);
      resolveNavigate(unlistenNavigate2);
    });

    expect(unlistenNewMail2).toHaveBeenCalled();
    expect(unlistenNavigate2).toHaveBeenCalled();
  });

  // --- Callback stability ---

  it("should not re-register listeners when callbacks change via re-render", async () => {
    // Given: hook is rendered with initial callbacks
    const onNewMail1 = vi.fn();
    const onNavigate1 = vi.fn();

    // Reset listen mock to track calls precisely
    mockListen.mockReset();
    const unlisten1 = vi.fn();
    const unlisten2 = vi.fn();
    mockListen.mockResolvedValueOnce(unlisten1).mockResolvedValueOnce(unlisten2);

    const { rerender } = renderHook(
      ({ onNewMail, onNavigateToMail }) =>
        useNewMailNotification({ onNewMail, onNavigateToMail }),
      {
        initialProps: { onNewMail: onNewMail1, onNavigateToMail: onNavigate1 },
      },
    );
    await act(async () => {});

    const initialListenCount = mockListen.mock.calls.length;

    // When: re-rendering with new callback references
    // (Hook should use refs internally to avoid re-subscribing)
    const onNewMail2 = vi.fn();
    const onNavigate2 = vi.fn();
    mockListen.mockResolvedValueOnce(vi.fn()).mockResolvedValueOnce(vi.fn());

    rerender({ onNewMail: onNewMail2, onNavigateToMail: onNavigate2 });
    await act(async () => {});

    // Then: listen is not called again (listeners are stable)
    expect(mockListen.mock.calls.length).toBe(initialListenCount);
  });
});
