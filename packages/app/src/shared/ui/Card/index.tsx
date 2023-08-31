import { FC, ReactNode } from "react";

import classNames from "classnames";
import styles from "./Card.module.scss";

export enum CardSize {
  M,
  L,
}

interface CardProps {
  size?: CardSize;
  children: string | ReactNode;
  className?: string;
}

export const Card: FC<CardProps> = ({
  size = CardSize.M,
  children,
  className,
}) => {
  return (
    <div className={classNames(styles.wrapper, className)}>{children}</div>
  );
};
