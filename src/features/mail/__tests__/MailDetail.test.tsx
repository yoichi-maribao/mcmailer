import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { MailDetail } from "../MailDetail";
import type { MessageDetail } from "../../../shared/types";

describe("MailDetail", () => {
  const htmlMessage: MessageDetail = {
    id: "msg1",
    subject: "HTML Email Test",
    from: "sender@gmail.com",
    date: "2026-04-01T10:00:00Z",
    body: "<html><body><p>Hello <strong>World</strong></p></body></html>",
    contentType: "text/html",
  };

  const plainTextMessage: MessageDetail = {
    id: "msg2",
    subject: "Plain Text Email",
    from: "sender2@gmail.com",
    date: "2026-04-01T11:00:00Z",
    body: "Hello World\nThis is a plain text email.\nLine 3.",
    contentType: "text/plain",
  };

  // --- Header rendering ---

  it("should display email subject", () => {
    // Given/When: rendering a message detail
    render(<MailDetail message={htmlMessage} />);

    // Then: subject is displayed
    expect(screen.getByText("HTML Email Test")).toBeInTheDocument();
  });

  it("should display sender email", () => {
    // Given/When: rendering a message detail
    render(<MailDetail message={htmlMessage} />);

    // Then: sender is displayed
    expect(screen.getByText("sender@gmail.com")).toBeInTheDocument();
  });

  it("should display date", () => {
    // Given/When: rendering a message detail
    render(<MailDetail message={htmlMessage} />);

    // Then: date is displayed (format may vary)
    expect(screen.getByText(/2026/)).toBeInTheDocument();
  });

  // --- HTML email rendering ---

  it("should render HTML email in a sandboxed iframe", () => {
    // Given/When: rendering an HTML email
    render(<MailDetail message={htmlMessage} />);

    // Then: an iframe is used with srcdoc and sandbox attributes
    const iframe = document.querySelector("iframe");
    expect(iframe).not.toBeNull();
    expect(iframe?.getAttribute("srcdoc")).toContain("Hello");
    expect(iframe?.getAttribute("sandbox")).toBeDefined();
  });

  it("should not use dangerouslySetInnerHTML for HTML emails", () => {
    // Given/When: rendering an HTML email
    const { container } = render(<MailDetail message={htmlMessage} />);

    // Then: no element has dangerouslySetInnerHTML (HTML content should be in iframe)
    const directHtml = container.querySelector("p > strong");
    expect(directHtml).toBeNull();
  });

  // --- Plain text email rendering ---

  it("should render plain text email in a pre element", () => {
    // Given/When: rendering a plain text email
    render(<MailDetail message={plainTextMessage} />);

    // Then: body is rendered in a pre element
    const preElement = document.querySelector("pre");
    expect(preElement).not.toBeNull();
    expect(preElement?.textContent).toContain("Hello World");
    expect(preElement?.textContent).toContain("This is a plain text email.");
  });

  it("should preserve line breaks in plain text email", () => {
    // Given/When: rendering a plain text email
    render(<MailDetail message={plainTextMessage} />);

    // Then: all lines are visible
    const preElement = document.querySelector("pre");
    expect(preElement?.textContent).toContain("Line 3.");
  });

  // --- No iframe for plain text ---

  it("should not render iframe for plain text email", () => {
    // Given/When: rendering a plain text email
    render(<MailDetail message={plainTextMessage} />);

    // Then: no iframe is present
    const iframe = document.querySelector("iframe");
    expect(iframe).toBeNull();
  });

  // --- Empty body ---

  it("should handle empty body gracefully", () => {
    // Given: a message with empty body
    const emptyMessage: MessageDetail = {
      id: "msg3",
      subject: "Empty Body",
      from: "sender@gmail.com",
      date: "2026-04-01T12:00:00Z",
      body: "",
      contentType: "text/plain",
    };

    // When: rendering
    render(<MailDetail message={emptyMessage} />);

    // Then: renders without error, subject still visible
    expect(screen.getByText("Empty Body")).toBeInTheDocument();
  });

  // --- No message selected ---

  it("should show placeholder when message is null", () => {
    // Given/When: rendering with null message (no selection)
    render(<MailDetail message={null} />);

    // Then: placeholder message is shown instead of loading
    expect(
      screen.getByText(/メールを選択|select.*mail|select.*message/i),
    ).toBeInTheDocument();
  });

  it("should not show email header elements when message is null", () => {
    // Given/When: rendering with null message
    render(<MailDetail message={null} />);

    // Then: no subject, sender, or date are displayed
    expect(screen.queryByRole("heading")).not.toBeInTheDocument();
  });
});
