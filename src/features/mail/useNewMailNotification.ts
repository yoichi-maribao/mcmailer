import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { EVENT_NEW_MAIL_RECEIVED, EVENT_NAVIGATE_TO_MAIL } from "../../shared/events";
import type { NewMailEvent, NavigateToMailEvent } from "../../shared/events";

interface UseNewMailNotificationOptions {
  onNewMail: (event: NewMailEvent) => void;
  onNavigateToMail: (event: NavigateToMailEvent) => void;
}

export function useNewMailNotification({ onNewMail, onNavigateToMail }: UseNewMailNotificationOptions) {
  const onNewMailRef = useRef(onNewMail);
  const onNavigateToMailRef = useRef(onNavigateToMail);

  onNewMailRef.current = onNewMail;
  onNavigateToMailRef.current = onNavigateToMail;

  useEffect(() => {
    let cancelled = false;
    let unlistenNewMailFn: (() => void) | null = null;
    let unlistenNavigateFn: (() => void) | null = null;

    listen<NewMailEvent>(EVENT_NEW_MAIL_RECEIVED, (event) => {
      onNewMailRef.current(event.payload);
    }).then((fn) => {
      if (cancelled) {
        fn();
      } else {
        unlistenNewMailFn = fn;
      }
    });

    listen<NavigateToMailEvent>(EVENT_NAVIGATE_TO_MAIL, (event) => {
      onNavigateToMailRef.current(event.payload);
    }).then((fn) => {
      if (cancelled) {
        fn();
      } else {
        unlistenNavigateFn = fn;
      }
    });

    return () => {
      cancelled = true;
      unlistenNewMailFn?.();
      unlistenNavigateFn?.();
    };
  }, []);
}
