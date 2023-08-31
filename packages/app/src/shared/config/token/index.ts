export const TOKEN = "token";

export const setToken = (token: string) => {
  localStorage.setItem(TOKEN, ["Bearer", token].join(" "));
};

export const getToken = (): string | null => {
  return localStorage.getItem(TOKEN);
};
