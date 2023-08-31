import { createEvent, createStore, sample } from "effector";
import { createQuery } from "@farfetched/core";

import { IText } from "../types";
import { getTextsByUserId } from "../api";
import { toastWarnFn } from "shared/config/toast";

export const $userTexts = createStore<IText[]>([]);
const $isWarned = createStore<boolean>(false);

export const getUserTextsFn = createEvent<number>();
const getUserTextsFx = createQuery<number, IText[]>({
  handler: getTextsByUserId,
});

sample({
  clock: getUserTextsFn,
  target: getUserTextsFx.start,
});

sample({
  clock: getUserTextsFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $userTexts,
});

sample({
  clock: getUserTextsFx.finished.failure,
  source: $isWarned,
  filter: (source) => {
    return !source;
  },
  fn: () => {
    return "You don't have any texts. It's time to create a new one";
  },
  target: toastWarnFn,
});

//TODO
sample({
  clock: getUserTextsFx.finished.finally,
  fn: () => {
    return true;
  },
  target: $isWarned,
});
