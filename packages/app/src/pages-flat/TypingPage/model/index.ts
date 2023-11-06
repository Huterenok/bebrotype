"use client";
//TODO
import { createQuery } from "@farfetched/core";
import { invoke } from "@withease/factories";
import { createEvent, createStore, sample } from "effector";

import { createKeyboard } from "enities/Keyboard";
import { IText, getRandomText, getTextById } from "enities/Text";

export const { $keyboardInput, $lastSymbol, inputEv, resetKeyboardEv } =
  invoke(createKeyboard);

export const $typingText = createStore<IText | null>(null);

export const getTypingTextEv = createEvent<number | undefined>();
const getTypingTextFx = createQuery<number | undefined, IText>({
  handler: async (p) => {
    return p ? getTextById(p) : getRandomText();
  },
});

sample({
  clock: getTypingTextEv,
  target: getTypingTextFx.start,
});

sample({
  clock: getTypingTextFx.finished.success,
  fn: (res) => {
    console.log(res.result);
    return res.result;
  },
  target: $typingText,
});

sample({
  clock: getTypingTextFx.finished.failure,
  fn: (clock) => {
    console.log(clock);
  },
});
