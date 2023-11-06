import { AuthResponse } from "../types";

import { createEndpointUrl, request } from "shared/api";

export const login = async () => {
  return await request<AuthResponse>({
    endpoint: "auth/login",
    method: "POST",
  });
};
export const register = async () => {
  return await request<AuthResponse>({
    endpoint: "auth/register",
    method: "POST",
  });
};

//TODO: todoooooo
export const googleOAuth = () => {
  location.replace(
    //TODO: return_url
    createEndpointUrl("auth/google-oauth?return_url=http://localhost:3000")
  );
};
