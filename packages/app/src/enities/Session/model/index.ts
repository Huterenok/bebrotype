import { createEvent, createStore, sample } from "effector";
import { createQuery } from "@farfetched/core";
import { invoke } from "@withease/factories";

import { ISession } from "../types";
import { whoami } from "../api";

import { createToken } from "shared/config";

export const $session = createStore<ISession | null>(null);
export const { $token: $sessionToken, setToken: setUserToken } = invoke(
  createToken<string>,
  {
    defaultValue: "",
    tokenIdent: "userToken",
  }
);

export const whoamiEv = createEvent();
const whoamiFx = createQuery<string, ISession>({
  handler: async (token) => {
    return await whoami(token);
  },
});

sample({
  clock: whoamiEv,
  source: $sessionToken,
  target: whoamiFx.start,
});

sample({
  clock: whoamiFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $session,
});
