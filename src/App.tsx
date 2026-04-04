import { useAuth } from "./features/auth/useAuth";
import { AuthScreen } from "./features/auth/AuthScreen";
import { MailScreen } from "./features/mail/MailScreen";

export default function App() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return <div role="progressbar">読み込み中...</div>;
  }

  if (!isAuthenticated) {
    return <AuthScreen />;
  }

  return <MailScreen />;
}
