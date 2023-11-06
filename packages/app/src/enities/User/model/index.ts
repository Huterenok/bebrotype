import { createEvent, createStore, sample } from "effector";
import { createQuery } from "@farfetched/core";

import { IUser } from "../types";
import { getUser } from "../api";

export const setUserEv = createEvent<IUser>();

export const getUserEv = createEvent<number>();
const getUserFx = createQuery({
  handler: getUser,
});

export const $profile = createStore<IUser | null>(null);

sample({
  clock: setUserEv,
  target: $profile,
});

sample({
  clock: getUserEv,
  target: getUserFx.start,
});

sample({
  clock: getUserFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $profile,
});
