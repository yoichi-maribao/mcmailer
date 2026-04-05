export const EVENT_NEW_MAIL_RECEIVED = "new-mail-received";
export const EVENT_NAVIGATE_TO_MAIL = "navigate-to-mail";

export interface NewMailEvent {
  accountEmail: string;
  messageId: string;
  subject: string;
  from: string;
}

export interface NavigateToMailEvent {
  accountEmail: string;
  messageId: string;
}
