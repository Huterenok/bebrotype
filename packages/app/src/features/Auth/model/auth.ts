import { createEffect } from "effector";

import { modalToggleFn } from "./modal";

import { $user } from "enities/User";

import {
  login,
  register,
  IRegisterRequest,
  ILoginRequest,
  googleOAuth,
} from "../api";

import { toast } from "react-toastify";
import { setToken } from "shared/lib";

export const registerFx = createEffect(
  async (registerRequest: IRegisterRequest) => {
    return await register(registerRequest);
  }
);
export const loginFx = createEffect(async (loginRequest: ILoginRequest) => {
  return await login(loginRequest);
});

export const googleOAuthFx = createEffect(async () => {
  return await googleOAuth();
});

$user.on(loginFx.doneData, (_, payload) => {
  toast.success("Successfully logged in!");
  setToken(payload.token);
  modalToggleFn();
  return payload.user;
});
$user.on(loginFx.failData, (_, payload) => {
  toast.error(payload.message);
});

$user.on(registerFx.doneData, (_, payload) => {
  toast.success("Successfully registered!");
  setToken(payload.token);
  modalToggleFn();
  return payload.user;
});
$user.on(registerFx.failData, (_, payload) => {
  toast.error(payload.message);
});

