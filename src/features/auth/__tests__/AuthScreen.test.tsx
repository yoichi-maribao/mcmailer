import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { AuthScreen } from "../AuthScreen";

const mockLogin = vi.fn();
const mockHasOAuthCredentials = vi.fn<() => Promise<boolean>>();
const mockSetOAuthCredentials = vi.fn<() => Promise<void>>();

vi.mock("../useAuth", () => ({
  useAuth: () => ({
    login: mockLogin,
    isLoading: false,
    error: null,
  }),
}));

vi.mock("../../../shared/commands", () => ({
  hasOAuthCredentials: (...args: unknown[]) => mockHasOAuthCredentials(...args),
  setOAuthCredentials: (...args: unknown[]) => mockSetOAuthCredentials(...args),
}));

describe("AuthScreen", () => {
  beforeEach(() => {
    mockLogin.mockReset();
    mockHasOAuthCredentials.mockReset();
    mockSetOAuthCredentials.mockReset();
  });

  it("should show OAuth setup form when credentials are not configured", async () => {
    mockHasOAuthCredentials.mockResolvedValueOnce(false);
    render(<AuthScreen />);

    await waitFor(() => {
      expect(screen.getByLabelText("Client ID")).toBeInTheDocument();
    });
    expect(screen.getByLabelText("Client Secret")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "保存" })).toBeInTheDocument();
  });

  it("should render login button when credentials are already configured", async () => {
    mockHasOAuthCredentials.mockResolvedValueOnce(true);
    render(<AuthScreen />);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i }),
      ).toBeInTheDocument();
    });
  });

  it("should save credentials and show login button", async () => {
    mockHasOAuthCredentials.mockResolvedValueOnce(false);
    mockSetOAuthCredentials.mockResolvedValueOnce(undefined);
    render(<AuthScreen />);

    await waitFor(() => {
      expect(screen.getByLabelText("Client ID")).toBeInTheDocument();
    });

    fireEvent.change(screen.getByLabelText("Client ID"), {
      target: { value: "test-client-id" },
    });
    fireEvent.change(screen.getByLabelText("Client Secret"), {
      target: { value: "test-client-secret" },
    });
    fireEvent.click(screen.getByRole("button", { name: "保存" }));

    await waitFor(() => {
      expect(mockSetOAuthCredentials).toHaveBeenCalledWith("test-client-id", "test-client-secret");
    });
    expect(
      screen.getByRole("button", { name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i }),
    ).toBeInTheDocument();
  });

  it("should call login when login button is clicked", async () => {
    mockHasOAuthCredentials.mockResolvedValueOnce(true);
    mockLogin.mockResolvedValueOnce(undefined);
    render(<AuthScreen />);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i }),
      ).toBeInTheDocument();
    });

    fireEvent.click(
      screen.getByRole("button", { name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i }),
    );

    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledTimes(1);
    });
  });

  it("should display error message when login fails", async () => {
    const mockUseAuth = await import("../useAuth");
    vi.mocked(mockUseAuth).useAuth = vi.fn().mockReturnValue({
      login: mockLogin,
      isLoading: false,
      error: "Authentication failed",
    });
    mockHasOAuthCredentials.mockResolvedValueOnce(true);
    render(<AuthScreen />);

    await waitFor(() => {
      expect(screen.getByText(/authentication failed/i)).toBeInTheDocument();
    });
  });

  it("should disable login button while loading", async () => {
    const mockUseAuth = await import("../useAuth");
    vi.mocked(mockUseAuth).useAuth = vi.fn().mockReturnValue({
      login: mockLogin,
      isLoading: true,
      error: null,
    });
    mockHasOAuthCredentials.mockResolvedValueOnce(true);

    render(<AuthScreen />);

    await waitFor(() => {
      const loginButton = screen.getByRole("button", {
        name: /gmail.*ログイン|sign.*in.*gmail|ログイン/i,
      });
      expect(loginButton).toBeDisabled();
    });
  });
});
