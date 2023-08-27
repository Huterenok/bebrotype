import { createEvent, createStore } from "effector";

export const $isModalOpened = createStore<boolean>(false);

export const modalToggleFn = createEvent();

$isModalOpened.on(modalToggleFn, (isOpened) => {
  return !isOpened;
});
