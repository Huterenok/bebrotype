import ky from "ky";
import { getToken } from "shared/lib/token";

export const API_URL = process.env.API_URL;

interface IRequest {
  endpoint: string;
  body?: unknown;
  method?: "POST" | "GET" | "PATCH";
}

const headers = new Headers();
headers.set("Content-Type", "application/json");

const connector = ky.create({
  prefixUrl: process.env.NEXT_PUBLIC_API_URL,
	throwHttpErrors: false
});

export async function request<R>(params: IRequest): Promise<R> {
  let { endpoint, method = "GET", body = {} } = params;

  let token = getToken();
  if (token) {
    headers.set("Authorization", token);
  }

  const response = await connector(endpoint, {
    method,
    body: JSON.stringify(body),
    headers,
  });

  if (!response.ok) {
    throw Error(await response.text());
  }

  return await response.json();
}
