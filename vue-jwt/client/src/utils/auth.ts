export const TOKEN_NAME = "my-vue-token";

export const LOGIN_URL = "/api/login";
export const CLIENT_LOGIN_URL = "/login";
export const REFRESH_URL = "/api/refresh";

export function login(tokens: string) {
  localStorage.setItem(TOKEN_NAME, tokens);
}

export function logout() {
  localStorage.removeItem(TOKEN_NAME);
}

export function isAuthenticated(): boolean {
  return !!localStorage.getItem(TOKEN_NAME);
}

export function getAccessToken(): string {
  const tokensStr = localStorage.getItem(TOKEN_NAME);

  if (tokensStr) {
    const tokens = JSON.parse(tokensStr);
    if (tokens && tokens.access_token)
      return tokens.access_token;
  }

  return '';
}

export function getRefreshToken(): string {
  const tokensStr = localStorage.getItem(TOKEN_NAME);

  if (tokensStr) {
    const tokens = JSON.parse(tokensStr);
    if (tokens && tokens.refresh_token)
      return tokens.refresh_token;
  }

  return '';
}
