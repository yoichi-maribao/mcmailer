import { useState, useEffect, useCallback, useRef } from "react";
import { listMessages, getMessage } from "../../shared/commands";
import type { MessageSummary, MessageDetail } from "../../shared/types";

interface MailsState {
  messages: MessageSummary[];
  hasMore: boolean;
  isLoading: boolean;
  isLoadingMore: boolean;
  error: string | null;
  loadMore: () => Promise<void>;
  refresh: () => Promise<void>;
  getMessageDetail: (id: string) => Promise<MessageDetail>;
}

export function useMails(): MailsState {
  const [messages, setMessages] = useState<MessageSummary[]>([]);
  const [hasMore, setHasMore] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const nextPageTokenRef = useRef<string | null>(null);

  const fetchMessages = useCallback(
    async (pageToken: string | null, append: boolean) => {
      console.log("[useMails] メール一覧を取得中...", { pageToken, append });
      const response = await listMessages(pageToken);
      console.log("[useMails] メール一覧を取得完了", {
        count: response.messages.length,
        hasMore: response.nextPageToken !== null,
      });
      if (append) {
        setMessages((prev) => [...prev, ...response.messages]);
      } else {
        setMessages(response.messages);
      }
      nextPageTokenRef.current = response.nextPageToken;
      setHasMore(response.nextPageToken !== null);
    },
    [],
  );

  useEffect(() => {
    console.log("[useMails] 初回読み込み開始");
    fetchMessages(null, false)
      .catch((e) => {
        const message = e instanceof Error ? e.message : String(e);
        console.error("[useMails] 読み込みエラー:", message);
        setError(message);
        setMessages([]);
      })
      .finally(() => {
        console.log("[useMails] 読み込み完了");
        setIsLoading(false);
      });
  }, [fetchMessages]);

  const loadMore = useCallback(async () => {
    if (!nextPageTokenRef.current || isLoadingMore) return;
    setIsLoadingMore(true);
    try {
      await fetchMessages(nextPageTokenRef.current, true);
    } finally {
      setIsLoadingMore(false);
    }
  }, [fetchMessages, isLoadingMore]);

  const refresh = useCallback(async () => {
    setMessages([]);
    nextPageTokenRef.current = null;
    await fetchMessages(null, false);
  }, [fetchMessages]);

  const getMessageDetail = useCallback(async (id: string) => {
    console.log("[useMails] メール詳細を取得中...", { id });
    const detail = await getMessage(id);
    console.log("[useMails] メール詳細を取得完了", { id, subject: detail.subject });
    return detail;
  }, []);

  return {
    messages,
    hasMore,
    isLoading,
    isLoadingMore,
    error,
    loadMore,
    refresh,
    getMessageDetail,
  };
}
