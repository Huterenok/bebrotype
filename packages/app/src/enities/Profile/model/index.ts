import { combine, createEvent, createStore, sample } from "effector";
import { createJsonQuery, declareParams } from "@farfetched/core";
import { zodContract } from "@farfetched/zod";

import { IProfile, ProfileContract } from "../types";
import { getProfile } from "../api";

export const setProfileEv = createEvent<IProfile>();
export const getProfileEv = createEvent<number>();
const getProfileFx = createJsonQuery({
  params: declareParams<number>(),
  request: {
    method: "GET",
    url: (id) => getProfile(id),
  },
  response: {
    contract: zodContract(ProfileContract),
  },
});

export const $profile = createStore<IProfile | null>(null);
export const $error = getProfileFx.$error;
export const $isPending = getProfileFx.$pending;

sample({
  clock: setProfileEv,
  target: $profile,
});

sample({
  clock: getProfileEv,
  target: getProfileFx.start,
});

sample({
  clock: getProfileFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $profile,
});
