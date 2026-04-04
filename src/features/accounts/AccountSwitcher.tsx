import { useState } from "react";
import type { AccountInfo } from "../../shared/types";

interface AccountSwitcherProps {
  accounts: AccountInfo[];
  activeAccount: AccountInfo;
  onSwitchAccount: (email: string) => void;
  onAddAccount: () => void;
}

export function AccountSwitcher({
  accounts,
  activeAccount,
  onSwitchAccount,
  onAddAccount,
}: AccountSwitcherProps) {
  const [isOpen, setIsOpen] = useState(false);

  const handleAccountClick = (email: string) => {
    if (email === activeAccount.email) return;
    onSwitchAccount(email);
    setIsOpen(false);
  };

  return (
    <div style={{ position: "relative" }}>
      <button
        aria-label="アカウント"
        onClick={() => setIsOpen((prev) => !prev)}
        style={{
          background: "none",
          border: "none",
          cursor: "pointer",
          fontSize: "1.2em",
          padding: "4px 8px",
        }}
      >
        {activeAccount.email[0].toUpperCase()}
      </button>
      {isOpen && (
        <div
          style={{
            position: "absolute",
            right: 0,
            top: "100%",
            backgroundColor: "#fff",
            border: "1px solid #ccc",
            borderRadius: "4px",
            minWidth: "200px",
            zIndex: 10,
          }}
        >
          <ul style={{ listStyle: "none", margin: 0, padding: 0 }}>
            {accounts.map((account) => (
              <li
                key={account.email}
                data-active={String(account.email === activeAccount.email)}
                onClick={() => handleAccountClick(account.email)}
                style={{
                  padding: "8px 12px",
                  cursor: "pointer",
                  fontWeight: account.email === activeAccount.email ? "bold" : "normal",
                }}
              >
                {account.email}
              </li>
            ))}
          </ul>
          <button
            onClick={onAddAccount}
            style={{
              display: "block",
              width: "100%",
              padding: "8px 12px",
              border: "none",
              borderTop: "1px solid #ccc",
              background: "none",
              cursor: "pointer",
              textAlign: "left",
            }}
          >
            アカウントを追加
          </button>
        </div>
      )}
    </div>
  );
}
