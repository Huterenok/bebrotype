import { ButtonHTMLAttributes, FC } from "react";
import styles from "./Button.module.scss";
import classNames from "classnames";

export enum ButtonColor {
  COMMON = "common",
  PRIME = "prime",
}

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  color?: ButtonColor;
}

export const Button: FC<ButtonProps> = ({
  children,
  className,
  color = ButtonColor.COMMON,
  type = "button",
  ...otherProps
}) => {
  return (
    <button
      type={type}
      className={classNames(
        styles.wrapper,
        { [styles[color]]: true },
        className
      )}
      {...otherProps}
    >
      {children}
    </button>
  );
};
