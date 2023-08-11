import { FC, ReactNode } from "react";

import styles from "./Card.module.scss";

export enum CardSize {
  M,
  L,
}

interface CardProps {
  size?: CardSize;
  children: string | ReactNode;
}

export const Card: FC<CardProps> = ({ size = CardSize.M, children }) => {
  return <div className={styles.wrapper}>{children}</div>;
};
