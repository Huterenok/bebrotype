import { createEffect, createEvent, createStore, sample } from "effector";

import { IUser } from "../types";
import { whoami } from "../api";

export const $user = createStore<IUser | null>(null);

export const whoamiFn = createEvent();

const whoamiFx = createEffect(async () => {
  return await whoami();
});

sample({
  clock: whoamiFn,
  target: whoamiFx,
});

$user.on(whoamiFx.doneData, (_, payload) => {
  console.log(payload);
  return payload;
});

$user.on(whoamiFx.failData, (state, err) => {
  return state;
});
