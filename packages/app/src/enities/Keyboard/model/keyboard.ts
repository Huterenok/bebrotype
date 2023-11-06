import { createFactory } from "@withease/factories";
import { createEvent, createStore, sample } from "effector";

export const createKeyboard = createFactory(() => {
  const inputEv = createEvent<string>();
  const resetKeyboardEv = createEvent();

  const $keyboardInput = createStore<string>("");
  //TODO
  const $lastSymbol = createStore<string>("~");

  $keyboardInput.reset(resetKeyboardEv);
  $lastSymbol.reset(resetKeyboardEv);

  sample({
    clock: inputEv,
    target: [$keyboardInput, $lastSymbol],
  });

  //TODO
  sample({
    clock: inputEv,
    fn: (input) => {
      //TODO
      return input === "" ? "~" : input[input.length - 1].toUpperCase();
    },
    target: $lastSymbol,
  });

  return {
    $keyboardInput,
    $lastSymbol,
    inputEv,
    resetKeyboardEv,
  };
});
