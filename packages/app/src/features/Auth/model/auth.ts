import { createEvent, sample } from "effector";
import { createMutation, createQuery } from "@farfetched/core";

import { modalToggleFn } from "./modal";
import { login, register, googleOAuth } from "../api";
import { ILoginRequest, IRegisterRequest, IUserResponse } from "../types";

import { $user } from "enities/User";

import { toastErrorFn, toastSuccessFn } from "shared/config/toast";
import { setToken } from "shared/config/token";

export const registerFn = createEvent<IRegisterRequest>();
const registerFx = createMutation<IRegisterRequest, IUserResponse>({
  handler: async (registerRequest) => {
    return await register(registerRequest);
  },
});

export const loginFn = createEvent<ILoginRequest>();
const loginFx = createMutation<ILoginRequest, IUserResponse>({
  handler: async (loginRequest: ILoginRequest) => {
    return await login(loginRequest);
  },
});

export const googleOAuthFn = createEvent();
const googleOAuthFx = createQuery({
  handler: async () => {
    await googleOAuth();
  },
});

sample({
  clock: registerFn,
  target: registerFx.start,
});

sample({
  clock: loginFn,
  target: loginFx.start,
});

sample({
  clock: googleOAuthFn,
  target: googleOAuthFx.start,
});

$user.on(registerFx.finished.success, (_, { result }) => {
  toastSuccessFn("Successfully registered!");
  setToken(result.token);
  modalToggleFn();
  return result.user;
});
$user.on(registerFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorFn(error.message);
});

$user.on(loginFx.finished.success, (_, { result }) => {
  toastSuccessFn("Successfully logged in!");
  setToken(result.token);
  modalToggleFn();
  return result.user;
});
$user.on(loginFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorFn(error.message);
});
