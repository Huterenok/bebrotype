import { createEvent, createStore } from "effector";
import { ReactNode } from "react";
import { toast } from "react-toastify";

const $toast = createStore(toast);

export const toastErrorEv = createEvent<string | ReactNode>();
export const toastSuccessEv = createEvent<string | ReactNode>();
export const toastWarnEv = createEvent<string | ReactNode>();

$toast.on(toastErrorEv, (state, payload) => {
  state.error(payload);
});
$toast.on(toastSuccessEv, (state, payload) => {
  state.success(payload);
});
$toast.on(toastWarnEv, (state, payload) => {
  state.warn(payload);
});
