import Image from "next/image";
import { FC } from "react";
import { useForm } from "effector-forms";
import { useUnit } from "effector-react";

import {
  authForm,
  $formCond,
  FormCondition,
  toggleFormCond,
  googleOAuthFx,
} from "../../../model";

import { Button, Input } from "shared/ui";

import styles from "./AuthForm.module.scss";

export const AuthForm: FC = () => {
  const { fields, submit } = useForm(authForm);
  const [formCond, toggleFormCondFn] = useUnit([$formCond, toggleFormCond]);
  const googleOAuth = useUnit(googleOAuthFx);

  const onSubmit = (e: React.MouseEvent) => {
    e.preventDefault();
    submit();
  };

  const onChangeFormCond = () => {
    toggleFormCondFn();
  };

  return (
    <div className={styles.wrapper}>
      <div className={styles.top}>
        <Button onClick={onChangeFormCond} className={styles.btn}>
          {formCond == FormCondition.LOGIN ? "To Registration" : "To Login"}
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
        {formCond == FormCondition.REGISTRATION && (
          <Input
            onChange={(e) => fields.username.onChange(e.target.value)}
            value={fields.username.value}
            placeholder="Username"
          />
        )}
        <Button onClick={onSubmit}>{"Let's start"}</Button>
        <h1>or</h1>
        <Image
          onClick={googleOAuth}
          width={48}
          height={48}
          src={"/icons/google.svg"}
          alt="Google Authorization"
          className={styles.google}
        />
      </form>
    </div>
  );
};
