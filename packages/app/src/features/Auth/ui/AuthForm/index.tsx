import { FC, useState } from "react";
import { useForm } from "effector-forms";
import { useUnit } from "effector-react";

import {
  authForm,
  $formCond,
  FormType,
  toggleFormCond,
} from "../../model";

import { Button, Input, Title } from "shared/ui";

import styles from "./AuthForm.module.scss";
import {} from "../../model/form";
import { toast } from "react-toastify";

export const AuthForm: FC = () => {
  const { fields, submit, hasError, errorText } = useForm(authForm);
  const [formCond, toggleFormCondFn] = useUnit([$formCond, toggleFormCond]);

  const onSubmit = (e: React.MouseEvent) => {
    e.preventDefault();
		//TODO: Better error displaying
    if (hasError("email")) {
      return toast.error(errorText("email"));
    } else if (hasError("password")) {
      return toast.error(errorText("password"));
    } else if (hasError("username")) {
      return toast.error(errorText("username"));
    }
    submit();
  };

  const onChangeFormCond = () => {
    toggleFormCondFn();
  };

  return (
    <div className={styles.wrapper}>
      <div className={styles.top}>
        <Button onClick={onChangeFormCond} className={styles.btn}>
          {formCond == FormType.LOGIN ? "To Registration" : "To Login"}
        </Button>
        <h1 className={styles.title}>{formCond.concat(" Form")}</h1>
      </div>
      <form className={styles.form}>
        <Input
          onChange={(e) => fields.email.onChange(e.target.value)}
          value={fields.email.value}
          placeholder="Email"
        />
        <Input
          onChange={(e) => fields.password.onChange(e.target.value)}
          value={fields.password.value}
          placeholder="Password"
        />
        {formCond == FormType.REGISTRATION && (
          <Input
            onChange={(e) => fields.username.onChange(e.target.value)}
            value={fields.username.value}
            placeholder="Username"
          />
        )}
        <Button onClick={onSubmit}>{"Let's start"}</Button>
      </form>
    </div>
  );
};
