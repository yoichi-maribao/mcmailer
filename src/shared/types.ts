export interface MessageSummary {
  id: string;
  subject: string;
  from: string;
  snippet: string;
  date: string;
  isUnread: boolean;
}

export interface MessageDetail {
  id: string;
  subject: string;
  from: string;
  date: string;
  body: string;
  contentType: string;
}

export interface MessageListResponse {
  messages: MessageSummary[];
  nextPageToken: string | null;
}

export interface AccountInfo {
  email: string;
  isActive: boolean;
}

export interface NotificationSettings {
  enabled: boolean;
  pubsubSubscription: string;
  pubsubTopic: string;
}
