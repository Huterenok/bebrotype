import { IUser } from "enities/User";

export interface IRegisterRequest {
	username: string;
	email: string;
	password: string;
}

export interface ILoginRequest {
	email: string;
	password: string;
}

export interface IUserResponse {
  token: string;
  user: IUser;
}