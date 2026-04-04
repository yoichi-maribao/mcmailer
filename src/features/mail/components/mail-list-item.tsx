import type { MessageSummary } from "../../../shared/types";

interface MailListItemProps {
  message: MessageSummary;
  onSelect: (id: string) => void;
  isSelected: boolean;
}

export function MailListItem({ message, onSelect, isSelected }: MailListItemProps) {
  return (
    <li
      data-selected={String(isSelected)}
      data-unread={String(message.isUnread)}
      onClick={() => onSelect(message.id)}
      style={{
        padding: "8px 12px",
        cursor: "pointer",
        backgroundColor: isSelected ? "#e8f0fe" : "transparent",
        borderBottom: "1px solid #e0e0e0",
      }}
    >
      <div style={{ fontWeight: message.isUnread ? "bold" : "normal" }}>
        {message.subject}
      </div>
      <div
        data-testid="snippet"
        style={{
          display: "-webkit-box",
          WebkitLineClamp: 2,
          WebkitBoxOrient: "vertical",
          overflow: "hidden",
          color: "#666",
          fontSize: "0.85em",
        }}
      >
        {message.snippet}
      </div>
    </li>
  );
}
