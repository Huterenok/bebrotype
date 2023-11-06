import { createFactory } from "@withease/factories";
import { createEvent, createStore } from "effector";

export const createModal = createFactory(() => {
  const $isModalOpened = createStore<boolean>(false);

  const modalToggleEv = createEvent();

  $isModalOpened.on(modalToggleEv, (isOpened) => {
    return !isOpened;
  });

  return {
    $isModalOpened,
    modalToggleEv,
  };
});
