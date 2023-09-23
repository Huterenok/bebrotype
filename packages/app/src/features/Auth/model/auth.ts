import { createEvent, sample } from "effector";
import {
  createJsonMutation,
  createQuery,
  declareParams,
} from "@farfetched/core";
import { zodContract } from "@farfetched/zod";

import { modalToggleEv } from "./modal";
import { login, register, googleOAuth } from "../api";
import {
  AuthResponseContract,
  ILoginRequest,
  IRegisterRequest,
} from "../types";

import { $user, setUserToken } from "enities/User";
import { toastErrorEv, toastSuccessEv } from "shared/config";

export const registerEv = createEvent<IRegisterRequest>();
const registerFx = createJsonMutation({
  params: declareParams<IRegisterRequest>(),
  request: {
    method: "POST",
    url: register,
    body: (params) => {
      return JSON.stringify(params);
    },
  },
  response: {
    contract: zodContract(AuthResponseContract),
  },
});

export const loginEv = createEvent<ILoginRequest>();
const loginFx = createJsonMutation({
  params: declareParams<ILoginRequest>(),
  request: {
    method: "POST",
    url: login
  },
  response: {
    contract: zodContract(AuthResponseContract),
  },
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

$user.on(registerFx.finished.success, (_, { result }) => {
  toastSuccessEv("Successfully registered!");
  setUserToken(result.token);
  modalToggleEv();
  return result.user;
});
$user.on(registerFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorEv(error.message);
});

$user.on(loginFx.finished.success, (_, { result }) => {
  toastSuccessEv("Successfully logged in!");
  setUserToken(result.token);
  modalToggleEv();
  return result.user;
});
$user.on(loginFx.finished.failure, (_, { error }) => {
  //@ts-ignore - TODO
  toastErrorEv(error.message);
});
