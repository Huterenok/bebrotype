import { createEffect, createEvent, createStore, sample } from "effector";
import { Form, createForm } from "effector-forms";
import { loginFx, registerFx } from "./user";

export enum FormType {
  LOGIN = "Login",
  REGISTRATION = "Registration",
}

export const toggleFormCond = createEvent();
export const $formCond = createStore<FormType>(FormType.LOGIN).on(
  toggleFormCond,
  (state) => {
    return state == FormType.LOGIN ? FormType.REGISTRATION : FormType.LOGIN;
  }
);

interface AuthForm {
  email: string;
  password: string;
  username: string;
}
//TODO: Validation
export const authForm = createForm({
  fields: {
    email: {
      init: "",
      rules: [
        {
          name: "email",
          validator: (value: string) => /\S+@\S+\.\S+/.test(value),
          errorText: "Email is required",
        },
      ],
    },
    password: {
      init: "",
      rules: [
        {
          name: "required",
          validator: (value: string) => Boolean(value),
          errorText: "Password is required",
        },
      ],
    },
    username: {
      init: "",
      rules: [
        {
          name: "username",
          validator: (value: string) => {
						//TODO: getState - bad practice
            if ($formCond.getState() == FormType.LOGIN) {
              return true;
            }
            return Boolean(value);
          },
          errorText: "Username is required",
        },
      ],
    },
  },
  validateOn: ["submit"],
});

export const formFx = createEffect((params: AuthForm) => {
  if ($formCond.getState() == FormType.LOGIN) {
    loginFx({ email: params.email, password: params.password });
  } else {
    registerFx(params);
  }
});

sample({
  clock: authForm.formValidated,
  target: formFx,
});
