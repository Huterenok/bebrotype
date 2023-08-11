import { ButtonHTMLAttributes, FC, ReactNode } from "react";
import styles from "./Button.module.scss";
import classNames from "classnames";

export enum ButtonSize {
  M,
  L,
}

export enum ButtonColor {
  COMMON = "common",
  PRIME = "prime",
}

interface ButtonProps {
  children: string | ReactNode;
  onClick?: (e: React.MouseEvent) => void;
  size?: ButtonSize;
  color?: ButtonColor;
  className?: string;
  type?: ButtonHTMLAttributes<HTMLButtonElement>["type"];
}

export const Button: FC<ButtonProps> = ({
  children,
  onClick,
  className,
  size = ButtonSize.M,
  color = ButtonColor.COMMON,
  type = "button",
}) => {
  return (
    <button
      type={type}
      onClick={onClick}
      className={classNames(
        styles.wrapper,
        { [styles[color]]: true },
        className
      )}
    >
      {children}
    </button>
  );
};
