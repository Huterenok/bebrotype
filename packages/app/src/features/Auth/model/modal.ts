import { createEvent, createStore } from "effector";

export const $isModalOpened = createStore<boolean>(false);

export const modalToggle = createEvent();

$isModalOpened.on(modalToggle, (isOpened) => !isOpened);
