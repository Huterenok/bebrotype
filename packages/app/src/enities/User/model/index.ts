import { createEvent, createStore, sample } from "effector";
import { createJsonQuery, declareParams } from "@farfetched/core";
import { zodContract } from "@farfetched/zod";
import { invoke } from "@withease/factories";

import { IUser, UserContract } from "../types";
import { whoami } from "../api";
import { createToken } from "shared/config";

export const $user = createStore<IUser | null>(null);
export const { $token: $userToken, setToken: setUserToken } = invoke(
  createToken<string>,
  {
    defaultValue: "",
    tokenIdent: "userToken",
  }
);

export const whoamiEv = createEvent();
const whoamiFx = createJsonQuery({
  params: declareParams<string>(),
  request: {
    method: "GET",
    url: whoami,
    headers: (token) => ({
      Authorization: `Bearer ${token}`,
    }),
  },
  response: {
    contract: zodContract(UserContract),
  },
});

sample({
  clock: whoamiEv,
  source: $userToken,
  target: whoamiFx.start,
});

sample({
  clock: whoamiFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $user,
});
