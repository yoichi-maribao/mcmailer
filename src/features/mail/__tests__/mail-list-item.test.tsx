import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MailListItem } from "../components/mail-list-item";
import type { MessageSummary } from "../../../shared/types";

describe("MailListItem", () => {
  const message: MessageSummary = {
    id: "msg1",
    subject: "Weekly Report Q1 2026",
    from: "manager@company.com",
    snippet:
      "Here is the weekly report for Q1 2026. Please review the attached spreadsheet and provide feedback by Friday.",
    date: "2026-04-01T10:00:00Z",
    isUnread: true,
  };

  const mockOnSelect = vi.fn();

  beforeEach(() => {
    mockOnSelect.mockReset();
  });

  // --- Subject rendering ---

  it("should display the email subject", () => {
    // Given: a message with a subject

    // When: rendering the mail list item
    render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: the subject is displayed
    expect(screen.getByText("Weekly Report Q1 2026")).toBeInTheDocument();
  });

  // --- Snippet preview (2 lines max) ---

  it("should display snippet text as a preview", () => {
    // Given: a message with a snippet

    // When: rendering the mail list item
    render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: snippet text is visible
    expect(
      screen.getByText(/Here is the weekly report for Q1 2026/),
    ).toBeInTheDocument();
  });

  it("should limit snippet preview to 2 lines via CSS", () => {
    // Given: a message with a long snippet

    // When: rendering the mail list item
    const { container } = render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: the snippet element has line-clamp style for 2 lines
    const snippetElement = container.querySelector("[data-testid='snippet']") as HTMLElement;
    expect(snippetElement).not.toBeNull();
    expect(snippetElement.style.webkitLineClamp).toBe("2");
  });

  // --- Selection ---

  it("should call onSelect with message id when clicked", () => {
    // Given: a rendered mail list item
    render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // When: clicking the item
    fireEvent.click(screen.getByText("Weekly Report Q1 2026"));

    // Then: onSelect is called with the message id
    expect(mockOnSelect).toHaveBeenCalledWith("msg1");
  });

  it("should visually indicate when selected", () => {
    // Given/When: rendering with isSelected=true
    const { container } = render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={true}
      />,
    );

    // Then: the item has a selected indicator
    const item = container.querySelector("[data-selected='true']");
    expect(item).not.toBeNull();
  });

  it("should not have selected indicator when not selected", () => {
    // Given/When: rendering with isSelected=false
    const { container } = render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: the item does not have selected indicator
    const item = container.querySelector("[data-selected='true']");
    expect(item).toBeNull();
  });

  // --- Unread indicator ---

  it("should visually distinguish unread messages", () => {
    // Given: an unread message

    // When: rendering
    const { container } = render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: the item has unread indicator
    const item = container.querySelector("[data-unread='true']");
    expect(item).not.toBeNull();
  });

  it("should not show unread indicator for read messages", () => {
    // Given: a read message
    const readMessage: MessageSummary = {
      ...message,
      isUnread: false,
    };

    // When: rendering
    const { container } = render(
      <MailListItem
        message={readMessage}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: the item does not have unread indicator
    const item = container.querySelector("[data-unread='true']");
    expect(item).toBeNull();
  });

  // --- Does not display sender (per plan: subject + 2-line snippet only) ---

  it("should not display sender in the list item", () => {
    // Given/When: rendering the mail list item
    render(
      <MailListItem
        message={message}
        onSelect={mockOnSelect}
        isSelected={false}
      />,
    );

    // Then: sender email is not shown in the list item
    // (タスク指示書: メールタイトル + 本文2行プレビューのみ)
    expect(screen.queryByText("manager@company.com")).not.toBeInTheDocument();
  });
});
