import ky from "ky";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export const createEndpointUrl = (endpoint: string) => `${API_URL}${endpoint}`;

interface IRequest {
  endpoint: string;
  body?: unknown;
  method?: "POST" | "GET" | "PATCH" | "DELETE";
  token?: string;
}

const connector = ky.create({
  prefixUrl: process.env.NEXT_PUBLIC_API_URL,
  throwHttpErrors: false,
});

export async function request<R>(params: IRequest): Promise<R> {
  let { endpoint, method = "GET", body = {}, token = "" } = params;

  const response = await connector(endpoint, {
    method,
    body: method == "GET" ? null : JSON.stringify(body),
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return await response.json();
}
