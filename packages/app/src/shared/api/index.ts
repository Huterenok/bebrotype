import ky from "ky";
import { getToken } from "shared/config/token";

export const API_URL = process.env.API_URL;

interface IRequest {
  endpoint: string;
  body?: unknown;
  method?: "POST" | "GET" | "PATCH" | "DELETE";
}

const defaultHeaders = new Headers();
defaultHeaders.set("Content-Type", "application/json");

const defaultFDHeaders = new Headers();
defaultFDHeaders.set("Content-Type", "multipart/form-data");

const connector = ky.create({
  prefixUrl: process.env.NEXT_PUBLIC_API_URL,
  throwHttpErrors: false,
});

export async function request<R>(params: IRequest): Promise<R> {
  let { endpoint, method = "GET", body = {} } = params;

  const token = getToken();
  if (token) {
    defaultHeaders.set("Authorization", token);
  }

  const response = await connector(endpoint, {
    method,
    body: method == "GET" ? null : JSON.stringify(body),
    headers: defaultHeaders,
  });

  if (!response.ok) {
    throw Error(await response.text());
  }

  return await response.json();
}

export async function requestFD<R>(params: IRequest): Promise<R> {
  let { endpoint, method = "PATCH", body = new FormData() } = params;

  const token = getToken();
  if (token) {
    defaultHeaders.set("Authorization", token);
  }

  const response = await connector(endpoint, {
    method,
    body: JSON.stringify(body),
    headers: defaultFDHeaders,
    redirect: "follow",
  });

  if (!response.ok) {
    throw Error(await response.text());
  }

  return await response.json();
}
