import { useState, useEffect } from "react";
import { useAuth } from "./useAuth";
import { setOAuthCredentials, hasOAuthCredentials } from "../../shared/commands";

export function AuthScreen() {
  const { login, isLoading, error } = useAuth();
  const [clientId, setClientId] = useState("");
  const [clientSecret, setClientSecret] = useState("");
  const [credentialsSaved, setCredentialsSaved] = useState(false);
  const [checking, setChecking] = useState(true);
  const [credentialsError, setCredentialsError] = useState<string | null>(null);

  useEffect(() => {
    hasOAuthCredentials()
      .then((has) => setCredentialsSaved(has))
      .finally(() => setChecking(false));
  }, []);

  const handleSaveCredentials = async () => {
    try {
      setCredentialsError(null);
      await setOAuthCredentials(clientId, clientSecret);
      setCredentialsSaved(true);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      setCredentialsError(message);
    }
  };

  if (checking) {
    return <div>読み込み中...</div>;
  }

  return (
    <div>
      <h1>McMailer</h1>

      {!credentialsSaved && (
        <div>
          <h2>OAuth設定</h2>
          <div>
            <label htmlFor="clientId">Client ID</label>
            <input
              id="clientId"
              type="text"
              value={clientId}
              onChange={(e) => setClientId(e.target.value)}
              placeholder="Google OAuth Client ID"
            />
          </div>
          <div>
            <label htmlFor="clientSecret">Client Secret</label>
            <input
              id="clientSecret"
              type="password"
              value={clientSecret}
              onChange={(e) => setClientSecret(e.target.value)}
              placeholder="Google OAuth Client Secret"
            />
          </div>
          <button
            onClick={handleSaveCredentials}
            disabled={!clientId || !clientSecret}
          >
            保存
          </button>
          {credentialsError && <p role="alert">{credentialsError}</p>}
        </div>
      )}

      {credentialsSaved && (
        <div>
          <button
            onClick={() => { login(); }}
            disabled={isLoading}
          >
            Gmailにログイン
          </button>
          {error && <p role="alert">{error}</p>}
          <button onClick={() => setCredentialsSaved(false)}>
            OAuth設定を変更
          </button>
        </div>
      )}
    </div>
  );
}
