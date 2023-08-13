import { request } from "shared/api";
import { ILoginRequest, IRegisterRequest, IUserResponse } from "./dto";

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

export type { ILoginRequest, IRegisterRequest, IUserResponse } from "./dto";