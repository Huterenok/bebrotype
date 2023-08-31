import { createEvent, createStore } from "effector";
import { ReactNode } from "react";
import { toast } from "react-toastify";

const $toast = createStore(toast);

export const toastErrorFn = createEvent<string | ReactNode>();
export const toastSuccessFn = createEvent<string | ReactNode>();

$toast.on(toastErrorFn, (state, payload) => {
  state.error(payload);
});
$toast.on(toastSuccessFn, (state, payload) => {
  state.success(payload);
});
