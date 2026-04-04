import type { MessageDetail } from "../../shared/types";

interface MailDetailProps {
  message: MessageDetail | null;
}

export function MailDetail({ message }: MailDetailProps) {
  if (!message) {
    return (
      <div style={{ padding: "40px", textAlign: "center", color: "#999" }}>
        メールを選択してください
      </div>
    );
  }

  return (
    <div style={{ padding: "16px", display: "flex", flexDirection: "column", height: "100%" }}>
      <header style={{ flexShrink: 0 }}>
        <h2 style={{ margin: "0 0 8px 0" }}>{message.subject}</h2>
        <div>{message.from}</div>
        <div>{message.date}</div>
      </header>
      {message.contentType === "text/html" ? (
        <iframe
          srcDoc={message.body}
          sandbox=""
          title="Email content"
          style={{ width: "100%", border: "none", flex: 1 }}
        />
      ) : (
        <pre style={{ flex: 1, overflow: "auto" }}>{message.body}</pre>
      )}
    </div>
  );
}
