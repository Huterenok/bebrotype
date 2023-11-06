import { createEvent, sample } from "effector";
import { createMutation, createQuery } from "@farfetched/core";

import { modalToggleEv } from "./modal";
import { login, register, googleOAuth } from "../api";
import { RegisterBody, LoginBody } from "../types";

import { $session, setUserToken } from "enities/Session";
import {
  createRouterInstace,
  toastErrorEv,
  toastSuccessEv,
} from "shared/config";

export const { navigateFx, routerGate } = createRouterInstace();

export const registerEv = createEvent<RegisterBody>();
const registerFx = createMutation({
  handler: register,
});

export const loginEv = createEvent<LoginBody>();
const loginFx = createMutation({
  handler: login,
});

export const googleOAuthEv = createEvent();
const googleOAuthFx = createQuery({
  handler: async () => {
    googleOAuth();
  },
});

sample({
  clock: registerEv,
  target: registerFx.start,
});

sample({
  clock: loginEv,
  target: loginFx.start,
});

sample({
  clock: googleOAuthEv,
  target: googleOAuthFx.start,
});

$session.on(registerFx.finished.success, (_, { result }) => {
  toastSuccessEv("Successfully registered!");
  setUserToken(result.token);
  modalToggleEv();
  navigateFx({ path: "/profile" });
  return result.user;
});
$session.on(registerFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorEv(error.message);
});

$session.on(loginFx.finished.success, (_, { result }) => {
  console.log("login success");

  toastSuccessEv("Successfully logged in!");
  setUserToken(result.token);
  modalToggleEv();
  navigateFx({ path: "/profile" });
  return result.user;
});
$session.on(loginFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorEv(error.message);
});
