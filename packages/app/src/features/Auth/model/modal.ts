import { createEvent, createStore } from "effector";

export const $isModalOpened = createStore<boolean>(false);

export const modalToggleEv = createEvent();

$isModalOpened.on(modalToggleEv, (isOpened) => {
  return !isOpened;
});
