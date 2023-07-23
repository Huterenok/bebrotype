import { FC, ReactNode } from "react";
import styles from "./Button.module.css";

export enum ButtonSize {
  M,
  L,
}

interface ButtonProps {
  size?: ButtonSize;
  children: string | ReactNode;
}

export const Button: FC<ButtonProps> = ({ size = ButtonSize.M, children }) => {
  return <button className={styles.wrapper}>{children}</button>;
};
