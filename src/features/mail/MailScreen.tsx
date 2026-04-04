import { useState, useEffect, useCallback } from "react";
import { MailList } from "./MailList";
import { MailDetail } from "./MailDetail";
import { HeaderBar } from "./components/header-bar";
import { useMails } from "./useMails";
import { useAccounts } from "../accounts/useAccounts";
import { useNewMailNotification } from "./useNewMailNotification";
import type { MessageDetail as MessageDetailType } from "../../shared/types";

export function MailScreen() {
  const { accounts, activeAccount, switchAccount, addAccount } = useAccounts();
  const { messages, hasMore, isLoading, isLoadingMore, loadMore, refresh, getMessageDetail } =
    useMails();
  const [selectedMessage, setSelectedMessage] =
    useState<MessageDetailType | null>(null);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  const handleSelect = useCallback(async (id: string) => {
    const detail = await getMessageDetail(id);
    setSelectedMessage(detail);
  }, [getMessageDetail]);

  useNewMailNotification({
    onNewMail: useCallback(() => {
      refresh();
    }, [refresh]),
    onNavigateToMail: useCallback((event) => {
      switchAccount(event.accountEmail)
        .then(() => refresh())
        .then(() => handleSelect(event.messageId))
        .catch(console.error);
    }, [switchAccount, refresh, handleSelect]),
  });

  const handleAccountSwitch = useCallback(
    (email: string) => {
      setSelectedMessage(null);
      // Fire-and-forget boundary: hook error state surfaces failures to UI
      switchAccount(email)
        .then(() => refresh())
        .catch(console.error);
    },
    [switchAccount, refresh],
  );

  const handleToggleSidebar = useCallback(() => {
    setIsSidebarOpen((prev) => !prev);
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.metaKey && e.key === "b") {
        e.preventDefault();
        handleToggleSidebar();
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handleToggleSidebar]);

  if (!activeAccount) return null;

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <HeaderBar
        accounts={accounts}
        activeAccount={activeAccount}
        onToggleSidebar={handleToggleSidebar}
        onSwitchAccount={handleAccountSwitch}
        onAddAccount={addAccount}
      />
      <main style={{ display: "flex", flex: 1, overflow: "hidden" }}>
        {isSidebarOpen && (
          <div style={{ flex: 3, borderRight: "1px solid #e0e0e0", overflow: "hidden" }}>
            <MailList
              messages={messages}
              onSelect={handleSelect}
              onLoadMore={loadMore}
              hasMore={hasMore}
              isLoading={isLoading}
              isLoadingMore={isLoadingMore}
              selectedId={selectedMessage?.id ?? null}
            />
          </div>
        )}
        <div style={{ flex: 7, overflow: "hidden" }}>
          <MailDetail message={selectedMessage} />
        </div>
      </main>
    </div>
  );
}
