import { IUser } from "enities/User";
import { UserContract } from "enities/User";
import { z } from "zod";

export interface IRegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface ILoginRequest {
  email: string;
  password: string;
}

export const AuthResponseContract = z.object({
  token: z.string(),
  user: UserContract,
});
