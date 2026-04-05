import { AccountSwitcher } from "../../accounts/AccountSwitcher";
import type { AccountInfo } from "../../../shared/types";

interface HeaderBarProps {
  accounts: AccountInfo[];
  activeAccount: AccountInfo;
  onToggleSidebar: () => void;
  onSwitchAccount: (email: string) => void;
  onAddAccount: () => void;
  onLogout: () => void;
}

export function HeaderBar({
  accounts,
  activeAccount,
  onToggleSidebar,
  onSwitchAccount,
  onAddAccount,
  onLogout,
}: HeaderBarProps) {
  return (
    <header
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "8px 16px",
        borderBottom: "1px solid #e0e0e0",
        backgroundColor: "#f8f9fa",
      }}
    >
      <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
        <button
          aria-label="サイドバー"
          onClick={onToggleSidebar}
          style={{
            background: "none",
            border: "none",
            cursor: "pointer",
            fontSize: "1.2em",
            padding: "4px 8px",
          }}
        >
          ☰
        </button>
        <span style={{ fontWeight: "bold", fontSize: "1.1em" }}>McMailer</span>
      </div>
      <AccountSwitcher
        accounts={accounts}
        activeAccount={activeAccount}
        onSwitchAccount={onSwitchAccount}
        onAddAccount={onAddAccount}
        onLogout={onLogout}
      />
    </header>
  );
}
