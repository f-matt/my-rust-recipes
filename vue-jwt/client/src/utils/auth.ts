export const TOKEN_NAME = "my-vue-token";

export function login(tokens: string) {
  localStorage.setItem(TOKEN_NAME, tokens);
}

export function logout() {
  localStorage.removeItem(TOKEN_NAME);
}

export function isAuthenticated(): boolean {
  return !!localStorage.getItem(TOKEN_NAME);
}
