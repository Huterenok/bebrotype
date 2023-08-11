import { TOKEN } from "shared/config/constants"

export const setToken = (token: string) => {
	localStorage.setItem(TOKEN, "Bearer " + token);
}

export const getToken = (): string | null => {
	return localStorage.getItem(TOKEN)
}