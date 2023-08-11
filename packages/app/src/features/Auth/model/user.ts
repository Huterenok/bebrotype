import {
  createStore,
  createEffect,
} from "effector";

import { IUser } from "enities/User";

import { modalToggle } from "./modal";

import { ILoginRequest, IRegisterRequest, IUserResponse } from "../types";

import { request } from "shared/api";
import { toast } from "react-toastify";
import { setToken } from "shared/lib/token";

export const $user = createStore<IUser | null>(null);

export const registerFx = createEffect(
  async (registerRequest: IRegisterRequest) => {
    let res = await request<IUserResponse>({
      endpoint: "auth/register",
      body: registerRequest,
      method: "POST",
    });
    return res;
  }
);
export const loginFx = createEffect(async (loginRequest: ILoginRequest) => {
  let res = await request<IUserResponse>({
    endpoint: "auth/login",
    body: loginRequest,
    method: "POST",
  });
  return res;
});

$user.on(loginFx.doneData, (_, payload) => {
  toast.success("Successfully registered!");
  setToken(payload.token);
  modalToggle();
  return payload?.user;
});
$user.on(registerFx.doneData, (_, payload) => {
  toast.success("Successfully logged in!");
  setToken(payload.token);
  modalToggle();
  return payload?.user;
});

$user.watch((state) => {
	console.log(state)
})

loginFx.failData.watch((payload) => {
  toast.error(payload.message);
});
registerFx.failData.watch((payload) => {
  toast.error(payload.message);
});
