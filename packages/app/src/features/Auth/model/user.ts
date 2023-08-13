import { createStore, createEffect } from "effector";

import { IUser } from "enities/User";

import { modalToggle } from "./modal";

import { login, register, IRegisterRequest, ILoginRequest } from "../api";

import { toast } from "react-toastify";
import { setToken } from "shared/lib/token";

export const $user = createStore<IUser | null>(null);

export const registerFx = createEffect(
  async (registerRequest: IRegisterRequest) => {
    return await register(registerRequest);
  }
);
export const loginFx = createEffect(async (loginRequest: ILoginRequest) => {
  return await login(loginRequest);
});

$user.on(loginFx.doneData, (_, payload) => {
	toast.success("Successfully logged in!");
  setToken(payload.token);
  modalToggle();
  return payload.user;
});
$user.on(registerFx.doneData, (_, payload) => {
  toast.success("Successfully registered!");
  setToken(payload.token);
  modalToggle();
  return payload.user;
});

$user.on(registerFx.failData, (_, payload) => {
  toast.error(payload.message);
});
$user.on(loginFx.failData, (_, payload) => {
  toast.error(payload.message);
});


