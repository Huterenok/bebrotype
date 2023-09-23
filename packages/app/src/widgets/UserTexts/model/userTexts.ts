import { createJsonQuery, declareParams } from "@farfetched/core";
import { zodContract } from "@farfetched/zod";
import { invoke } from "@withease/factories";
import { createEvent, sample } from "effector";

import {
  createTextList,
  getTextsByUserId,
  TextArrayContract,
} from "enities/Text";

const userTexts = invoke(createTextList, {});

export const getUserTextsEv = createEvent<number>();

const getUserTextsFx = createJsonQuery({
  params: declareParams<number>(),
  request: {
    method: "GET",
    url: getTextsByUserId,
  },
  response: {
    contract: zodContract(TextArrayContract),
  },
});

export const $userTexts = userTexts.$texts;
export const $error = getUserTextsFx.$error;
export const $isPending = getUserTextsFx.$pending;

sample({
  clock: getUserTextsEv,
  target: getUserTextsFx.start,
});

sample({
  clock: getUserTextsFx.finished.success,
  fn: (clock) => {
    return clock.result;
  },
  target: $userTexts,
});
