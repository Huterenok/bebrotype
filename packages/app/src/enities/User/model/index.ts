import { createEvent, createStore, sample } from "effector";

import { IUpdateUser, IUser } from "../types";
import { whoami } from "../api";
import { createQuery } from "@farfetched/core";

export const $user = createStore<IUser | null>(null);

export const whoamiFn = createEvent();
const whoamiFx = createQuery({
  handler: whoami,
});

//TODO
export const updateUserFn = createEvent<IUser>();
sample({
  clock: updateUserFn,
  target: $user,
});

sample({
  clock: whoamiFn,
  target: whoamiFx.start,
});

sample({
  clock: whoamiFx.$data,
  target: $user,
});
