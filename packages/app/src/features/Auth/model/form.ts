import {
  combine,
  createEffect,
  createEvent,
  createStore,
  sample,
} from "effector";
import { ValidationError, createForm } from "effector-forms";

import { loginFn, registerFn } from "./auth";

import { toast } from "react-toastify";
import { trimObject } from "shared/lib";

export enum FormCondition {
  LOGIN = "Login",
  REGISTRATION = "Registration",
}

export const toggleFormCond = createEvent();
export const $formCond = createStore<FormCondition>(FormCondition.LOGIN).on(
  toggleFormCond,
  (state) => {
    return state == FormCondition.LOGIN
      ? FormCondition.REGISTRATION
      : FormCondition.LOGIN;
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
          errorText: "Email must be valid",
        },
      ],
    },
    password: {
      init: "",
      rules: [
        {
          name: "required",
          validator: (value: string) => Boolean(value),
          errorText: "Password must consist of 6 to 24 characters",
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
            if ($formCond.getState() == FormCondition.LOGIN) {
              return true;
            }
            return Boolean(value);
          },
          errorText: "Username must consist of 6 to 24 characters",
        },
      ],
    },
  },
  validateOn: ["submit"],
});

type Errors = [...(ValidationError<string> | null)[]];

const displayErrorsFx = createEffect((errors: Errors) => {
  errors.map((err) => {
    if (err) {
      toast.error(err.errorText);
    }
  });
});

sample({
  clock: authForm.submit,
  source: [
    authForm.fields.email.$firstError,
    authForm.fields.username.$firstError,
    authForm.fields.password.$firstError,
  ],
  target: displayErrorsFx,
});

const formValidatedFx = createEffect(
  async (arr: (AuthForm | FormCondition)[]) => {
    const formCond = arr[0];
    //TODO: type this
    const formData = trimObject(arr[1]) as AuthForm;
    if (formCond == FormCondition.LOGIN) {
      loginFn({
        email: formData.email,
        password: formData.password,
      });
    } else {
      console.log(formData);
      registerFn(formData);
    }
  }
);

sample({
  clock: authForm.formValidated,
  source: [$formCond, authForm.$values],
  target: formValidatedFx,
});
