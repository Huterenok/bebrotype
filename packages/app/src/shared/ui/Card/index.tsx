import { FC, ReactNode } from "react";

import classNames from "classnames";
import styles from "./Card.module.scss";

interface CardProps {
  children: string | ReactNode;
  className?: string;
}

export const Card: FC<CardProps> = ({ children, className }) => {
  return (
    <div className={classNames(styles.wrapper, className)}>{children}</div>
  );
};
