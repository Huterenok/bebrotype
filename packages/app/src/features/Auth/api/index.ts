import { ILoginRequest, IRegisterRequest, IUserResponse } from "../types";

import { request } from "shared/api";

export const register = async (
  registerRequest: IRegisterRequest
): Promise<IUserResponse> => {
  return await request<IUserResponse>({
    endpoint: "auth/register",
    body: registerRequest,
    method: "POST",
  });
};

export const login = async (
  loginRequest: ILoginRequest
): Promise<IUserResponse> => {
  return await request<IUserResponse>({
    endpoint: "auth/login",
    body: loginRequest,
    method: "POST",
  });
};

//TODO: todoooooo
export const googleOAuth = async () => {
  location.replace(
    //TODO: return url
    "http://localhost:3001/api/auth/google-oauth?return_url=http://localhost:3000"
  );
};
