import { useEffect, useRef } from "react";
import { MailListItem } from "./components/mail-list-item";
import type { MessageSummary } from "../../shared/types";

interface MailListProps {
  messages: MessageSummary[];
  onSelect: (id: string) => void;
  onLoadMore: () => void;
  onRefresh: () => void;
  hasMore: boolean;
  isLoading: boolean;
  isLoadingMore: boolean;
  selectedId: string | null;
}

export function MailList({
  messages,
  onSelect,
  onLoadMore,
  onRefresh,
  hasMore,
  isLoading,
  isLoadingMore,
  selectedId,
}: MailListProps) {
  const sentinelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!sentinelRef.current || !hasMore) return;
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) onLoadMore();
    });
    observer.observe(sentinelRef.current);
    return () => observer.disconnect();
  }, [hasMore, onLoadMore]);

  if (isLoading) {
    return <div role="progressbar">読み込み中...</div>;
  }

  if (messages.length === 0) {
    return <div>メールがありません</div>;
  }

  return (
    <div style={{ overflowY: "auto", height: "100%" }}>
      <div style={{ padding: "8px", textAlign: "right" }}>
        <button
          aria-label="リロード"
          onClick={onRefresh}
          style={{
            background: "none",
            border: "1px solid #ccc",
            borderRadius: "4px",
            cursor: "pointer",
            padding: "4px 12px",
            fontSize: "13px",
          }}
        >
          リロード
        </button>
      </div>
      <ul style={{ listStyle: "none", margin: 0, padding: 0 }}>
        {messages.map((msg) => (
          <MailListItem
            key={msg.id}
            message={msg}
            onSelect={onSelect}
            isSelected={msg.id === selectedId}
          />
        ))}
      </ul>
      {hasMore && <div ref={sentinelRef} data-testid="scroll-sentinel" />}
      {isLoadingMore && (
        <div
          role="progressbar"
          style={{
            padding: "12px",
            textAlign: "center",
            color: "#888",
            fontSize: "13px",
          }}
        >
          読み込み中...
        </div>
      )}
    </div>
  );
}
